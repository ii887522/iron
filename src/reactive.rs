use crate::IDManager;
use crate::Readonly;
use crate::Shared;
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

struct Child<T: ?Sized, R: ?Sized + Debug = dyn Debug> {
  reactive: Shared<Reactive<R>>,
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
      .finish_non_exhaustive()
  }
}

#[derive(Debug)]
pub struct Handle<T: ?Sized + Debug>(Shared<Reactive<dyn Debug>>, PhantomData<T>);

impl<T: ?Sized + Debug> Handle<T> {
  fn new(value: Shared<Reactive<dyn Debug>>) -> Self {
    Self(value, PhantomData)
  }

  pub fn borrow(&self) -> Ref<Reactive<T>> {
    unsafe { transmute::<_, &Shared<_>>(&self.0) }.borrow()
  }

  pub fn borrow_mut(&self) -> RefMut<Reactive<T>> {
    unsafe { transmute::<_, &Shared<_>>(&self.0) }.borrow_mut()
  }
}

impl<T: ?Sized + Debug> Clone for Handle<T> {
  fn clone(&self) -> Self {
    Self(Rc::clone(&self.0), PhantomData)
  }
}

impl<T: Debug + 'static> Handle<T> {
  pub fn from(value: T) -> Self {
    Self(
      Rc::new(RefCell::new(Reactive::new(Box::new(value), None))),
      PhantomData,
    )
  }
}

impl<T: ?Sized + Debug> From<Shared<Reactive<dyn Debug>>> for Handle<T> {
  fn from(value: Shared<Reactive<dyn Debug>>) -> Self {
    Self(value, PhantomData)
  }
}

impl<T: ?Sized + Debug> From<Reactive<dyn Debug>> for Handle<T> {
  fn from(value: Reactive<dyn Debug>) -> Self {
    Self(Rc::new(RefCell::new(value)), PhantomData)
  }
}

/// It is a wrapper that listens for changes to the value being held and automatically notifies all registered watchers
/// about the new value given. It is used to establish communications between multiple modules to achieve loose coupling
/// between the modules involved.
#[derive(Debug)]
pub struct Reactive<T: ?Sized + Debug> {
  watcher_id_manager: IDManager,
  id: Readonly<usize>,
  watcher_id: Option<usize>,
  children: Vec<Option<Child<T>>>,
  value: Box<T>,
}

impl<T: ?Sized + Debug> Reactive<T> {
  fn new(value: Box<T>, watcher_id: Option<usize>) -> Self {
    static mut ID_MANAGER: IDManager = IDManager::new();
    Self {
      watcher_id_manager: IDManager::new(),
      id: unsafe { ID_MANAGER.next() }.unwrap().into(),
      watcher_id,
      children: vec![],
      value,
    }
  }

  pub fn get_id(&self) -> usize {
    *self.id
  }

  pub fn get_value(&self) -> &T {
    &self.value
  }

  fn set_boxed_value(&mut self, value: Box<T>) {
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
        .set_boxed_value((child.on_change)(&*self.value));
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
    mut on_change: impl FnMut(&T) -> U + 'static,
  ) -> Handle<U> {
    let watcher_id = self.watcher_id_manager.next().unwrap();
    let reactive: Shared<Reactive<dyn Debug>> = Rc::new(RefCell::new(Reactive::new(
      Box::new(on_change(&self.value)),
      Some(watcher_id),
    )));
    let child: Child<_, dyn Debug> = Child {
      on_change: Box::new(move |value| Box::new(on_change(value))),
      reactive: Rc::clone(&reactive),
    };
    if watcher_id == self.children.len() {
      self.children.push(Some(child));
    } else {
      self.children[watcher_id] = Some(child);
    }
    Handle::new(reactive)
  }

  /// It detaches the registered watcher associated with the `reactive` given so the watcher no longer reacts to
  /// changes of the value held by this reactive object.
  ///
  /// `reactive`: The transformed reactive that is associated with the watcher to be unwatched.
  pub fn unwatch<U: ?Sized + Debug + 'static>(&mut self, reactive: Handle<U>) {
    let watcher_id = reactive.borrow().watcher_id.unwrap();
    self.children[watcher_id] = None;
    self.watcher_id_manager.free(watcher_id).unwrap();
  }
}

impl<T: Debug> Reactive<T> {
  pub fn set_value(&mut self, value: T) {
    self.set_boxed_value(Box::new(value));
  }
}

impl<T: ?Sized + Debug> From<Handle<T>> for Shared<Reactive<dyn Debug>> {
  fn from(value: Handle<T>) -> Self {
    value.0
  }
}

impl<T: ?Sized + Debug> From<Handle<T>> for Reactive<dyn Debug> {
  fn from(value: Handle<T>) -> Self {
    Rc::try_unwrap(value.0)
      .expect(
        "value should be the only reactive handle instance to a particular reactive object exists at the current moment!"
      )
      .into_inner()
  }
}
