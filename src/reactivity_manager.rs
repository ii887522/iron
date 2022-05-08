use crate::reactive;
use crate::Reactive;
use crate::Shared;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

/// It is a manager that allows users to register watchers more conveniently and unwatch all registered watchers at
/// once.
#[derive(Debug, Default)]
pub struct ReactivityManager {
  watcher_reactives: HashMap<usize, Vec<Shared<Reactive<dyn Debug>>>>,
  reactives: HashMap<usize, Shared<Reactive<dyn Debug>>>,
}

impl ReactivityManager {
  pub fn new() -> Self {
    Self {
      watcher_reactives: HashMap::new(),
      reactives: HashMap::new(),
    }
  }

  /// It registers a new watcher that transforms the reactive object received into a new reactive object that holds a
  /// different type.
  ///
  /// `reactive`: The reactive object which its value changes are listened for.
  ///
  /// `on_change`: It is a watcher that listens for new value and processes it.
  ///
  /// It returns a transformed reactive object.
  pub fn watch<A: ?Sized + Debug + 'static, R: Debug + 'static>(
    &mut self,
    reactive: reactive::Handle<A>,
    on_change: impl FnMut(&A) -> R + 'static,
  ) -> reactive::Handle<R> {
    let reactive_id = reactive.borrow().get_id();
    self
      .reactives
      .entry(reactive_id)
      .or_insert_with(|| reactive.clone().into());
    let result = reactive.borrow_mut().watch(on_change);
    self
      .watcher_reactives
      .entry(reactive_id)
      .or_insert(vec![])
      .push(result.clone().into());
    result
  }

  /// It registers a new watcher that transforms these reactive objects received into a new reactive object that holds a
  /// different type.
  ///
  /// `reactive_a`: The first reactive object which its value changes are listened for.
  ///
  /// `reactive_b`: The second reactive object which its value changes are listened for.
  ///
  /// `on_change`: It is a watcher that listens for new value and processes it.
  ///
  /// It returns a transformed reactive object.
  pub fn watch2<A: ?Sized + Debug + 'static, B: ?Sized + Debug + 'static, R: Debug + 'static>(
    &mut self,
    reactive_a: reactive::Handle<A>,
    reactive_b: reactive::Handle<B>,
    mut on_change: impl FnMut(&A, &B) -> R + 'static,
  ) -> reactive::Handle<R> {
    let result = reactive::Handle::from(on_change(
      reactive_a.borrow().get_value(),
      reactive_b.borrow().get_value(),
    ));
    let on_change = Rc::new(RefCell::new(on_change));
    {
      let reactive_a = reactive_a.clone();
      let reactive_b = reactive_b.clone();
      let result = result.clone();
      let on_change = Rc::clone(&on_change);
      self.watch(reactive_a, move |value| {
        result.borrow_mut().set_value(on_change.borrow_mut()(
          value,
          reactive_b.borrow().get_value(),
        ));
      });
    }
    {
      let reactive_a = reactive_a.clone();
      let reactive_b = reactive_b.clone();
      let result = result.clone();
      let on_change = Rc::clone(&on_change);
      self.watch(reactive_b, move |value| {
        result.borrow_mut().set_value(on_change.borrow_mut()(
          reactive_a.borrow().get_value(),
          value,
        ));
      });
    }
    result
  }

  /// It registers a new watcher that transforms these reactive objects received into a new reactive object that holds a
  /// different type.
  ///
  /// `reactive_a`: The first reactive object which its value changes are listened for.
  ///
  /// `reactive_b`: The second reactive object which its value changes are listened for.
  ///
  /// `reactive_c`: The third reactive object which its value changes are listened for.
  ///
  /// `on_change`: It is a watcher that listens for new value and processes it.
  ///
  /// It returns a transformed reactive object.
  pub fn watch3<
    A: ?Sized + Debug + 'static,
    B: ?Sized + Debug + 'static,
    C: ?Sized + Debug + 'static,
    R: Debug + 'static,
  >(
    &mut self,
    reactive_a: reactive::Handle<A>,
    reactive_b: reactive::Handle<B>,
    reactive_c: reactive::Handle<C>,
    mut on_change: impl FnMut(&A, &B, &C) -> R + 'static,
  ) -> reactive::Handle<R> {
    let result = reactive::Handle::from(on_change(
      reactive_a.borrow().get_value(),
      reactive_b.borrow().get_value(),
      reactive_c.borrow().get_value(),
    ));
    let on_change = Rc::new(RefCell::new(on_change));
    {
      let reactive_a = reactive_a.clone();
      let reactive_b = reactive_b.clone();
      let reactive_c = reactive_c.clone();
      let result = result.clone();
      let on_change = Rc::clone(&on_change);
      self.watch(reactive_a, move |value| {
        result.borrow_mut().set_value(on_change.borrow_mut()(
          value,
          reactive_b.borrow().get_value(),
          reactive_c.borrow().get_value(),
        ));
      });
    }
    {
      let reactive_a = reactive_a.clone();
      let reactive_b = reactive_b.clone();
      let reactive_c = reactive_c.clone();
      let result = result.clone();
      let on_change = Rc::clone(&on_change);
      self.watch(reactive_b, move |value| {
        result.borrow_mut().set_value(on_change.borrow_mut()(
          reactive_a.borrow().get_value(),
          value,
          reactive_c.borrow().get_value(),
        ));
      });
    }
    {
      let reactive_a = reactive_a.clone();
      let reactive_b = reactive_b.clone();
      let reactive_c = reactive_c.clone();
      let result = result.clone();
      let on_change = Rc::clone(&on_change);
      self.watch(reactive_c, move |value| {
        result.borrow_mut().set_value(on_change.borrow_mut()(
          reactive_a.borrow().get_value(),
          reactive_b.borrow().get_value(),
          value,
        ));
      });
    }
    result
  }
}

impl Drop for ReactivityManager {
  fn drop(&mut self) {
    for reactive_id in self.watcher_reactives.keys() {
      let mut reactive = self.reactives.get(reactive_id).unwrap().borrow_mut();
      for watcher_reactive in self.watcher_reactives.get(reactive_id).unwrap() {
        reactive.unwatch::<dyn Debug>(Rc::clone(watcher_reactive).into());
      }
    }
  }
}
