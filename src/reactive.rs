use crate::IDManager;
use crate::Readonly;
use std::any::type_name;
use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::mem::transmute;
use std::rc::Rc;

static mut ID_MANAGER: IDManager = IDManager::new();

struct Child<T: ?Sized, R: ?Sized + Debug = dyn Debug> {
  reactive: Rc<RefCell<Reactive<R>>>,
  on_change: Box<dyn FnMut(&T) -> Box<R>>,
}

impl<T: ?Sized, R: ?Sized + Debug> Debug for Child<T, R> {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    formatter
      .debug_struct(&format!(
        "Child<{}, {}>",
        type_name::<T>(),
        type_name::<R>()
      ))
      .field("reactive", &self.reactive)
      .finish()
  }
}

#[derive(Debug)]
pub struct Holder<T: ?Sized + Debug> {
  value: Rc<RefCell<Reactive<dyn Debug>>>,
  phantom: PhantomData<T>,
}

impl<T: ?Sized + Debug> Holder<T> {
  fn new(value: Rc<RefCell<Reactive<dyn Debug>>>) -> Self {
    Self {
      value,
      phantom: PhantomData,
    }
  }

  pub fn borrow(&self) -> Ref<Reactive<T>> {
    unsafe { transmute::<_, &Rc<RefCell<Reactive<T>>>>(&self.value) }.borrow()
  }

  pub fn borrow_mut(&mut self) -> RefMut<Reactive<T>> {
    unsafe { transmute::<_, &Rc<RefCell<Reactive<T>>>>(&self.value) }.borrow_mut()
  }
}

impl<T: ?Sized + Debug> Clone for Holder<T> {
  fn clone(&self) -> Self {
    Self {
      value: Rc::clone(&self.value),
      phantom: PhantomData,
    }
  }
}

/// It is a wrapper that listens for changes to the value being held and automatically notifies all registered watchers
/// about the new value given. It is used to establish communications between multiple modules to achieve loose coupling
/// between the modules involved.
#[derive(Debug)]
pub struct Reactive<T: ?Sized + Debug> {
  watcher_id_manager: IDManager,
  pub id: Readonly<usize>,
  watcher_id: Readonly<Option<usize>>,
  value: Box<T>,
  children: Vec<Option<Child<T>>>,
}

impl<T: ?Sized + Debug> Reactive<T> {
  fn new(value: Box<T>, watcher_id: Option<usize>) -> Self {
    Self {
      watcher_id_manager: IDManager::new(),
      id: unsafe { ID_MANAGER.next() }.unwrap().into(),
      watcher_id: watcher_id.into(),
      value,
      children: vec![],
    }
  }

  /// `value`: The initial value to be held by this wrapper.
  pub fn from(value: Box<T>) -> Self {
    Self::new(value, None)
  }

  pub fn get_value(&self) -> &T {
    &self.value
  }

  pub fn set_value(&mut self, value: Box<T>) {
    if format!("{:?}", &*self.value) == format!("{:?}", &*value) {
      return;
    }
    self.value = value;
    self.wake_children();
  }

  /// It notifies all other reactive objects associated with their watchers about the new value given just now.
  fn wake_children(&mut self) {
    for child in &mut self.children.iter_mut().flatten() {
      child
        .reactive
        .borrow_mut()
        .set_value((child.on_change)(&*self.value));
    }
  }

  /// It registers a new watcher that transforms this reactive object into a new reactive object that holds a different
  /// type.
  ///
  /// `on_change`: It is a watcher that listens for new value and processes it.
  ///
  /// It returns a transformed reactive object that can be unwatched on in the future.
  pub fn watch<U: Debug + 'static>(
    &mut self,
    mut on_change: impl FnMut(&T) -> Box<U> + 'static,
  ) -> Holder<U> {
    let watcher_id = self.watcher_id_manager.next().unwrap();
    let reactive: Rc<RefCell<Reactive<dyn Debug>>> = Rc::new(RefCell::new(Reactive::new(
      on_change(&*self.value),
      Some(watcher_id),
    )));
    let child: Child<_, dyn Debug> = Child {
      on_change: Box::new(move |value| on_change(value)),
      reactive: Rc::clone(&reactive),
    };
    if watcher_id == self.children.len() {
      self.children.push(Some(child));
    } else {
      self.children[watcher_id] = Some(child);
    }
    Holder::new(reactive)
  }

  /// It detaches the registered watcher associated with the `reactive_holder` given so the watcher no longer reacts to
  /// changes of the value helded by this reactive object.
  ///
  /// `reactive_holder`: The transformed reactive holder that is associated with the watcher to be unwatched.
  pub fn unwatch<U: Debug>(&mut self, reactive_holder: Holder<U>) {
    let watcher_id = reactive_holder.borrow().watcher_id.unwrap();
    self.children[watcher_id] = None;
    self.watcher_id_manager.free(watcher_id).unwrap();
  }
}
