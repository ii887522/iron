use crate::{any::reactive, Reactive};

use std::{
  cell::{Ref, RefMut},
  fmt::Debug,
  vec,
};

#[derive(Debug)]
pub struct Arg<T>(Vec<T>);

impl<T> From<()> for Arg<T> {
  fn from(_: ()) -> Self {
    Self(vec![])
  }
}

impl<T> From<Vec<T>> for Arg<T> {
  fn from(value: Vec<T>) -> Self {
    Self(value)
  }
}

/// It is a wrapper that listens for changes to the array being held and automatically notifies all registered watchers
/// about the new values in the array given. It is used to establish communications between multiple modules to achieve
/// loose coupling between the modules involved.
#[derive(Debug)]
pub struct ReactiveSlice<T> {
  proto: reactive::Handle<Vec<T>>,
}

impl<T> ReactiveSlice<T> {
  pub fn borrow(&self) -> Ref<Reactive<Vec<T>>> {
    self.proto.borrow()
  }

  pub fn borrow_mut(&self) -> RefMut<Reactive<Vec<T>>> {
    self.proto.borrow_mut()
  }
}

impl<T: Debug + 'static> ReactiveSlice<T> {
  pub fn new(value: impl Into<Arg<T>>) -> Self {
    Self {
      proto: reactive::Handle::from(value.into().0),
    }
  }
}

impl<T> Reactive<Vec<T>> {
  /// It replaces an old item located at the `id` with the new `item`.
  ///
  /// `id`: The position for which the item needs to be replaced.
  ///
  /// `item`: The new item located at the `id`.
  pub fn set(&mut self, id: usize, item: T) {
    // SAFETY: Changing the inner value before waking up all the child watchers does not violate the invariants of the
    // called functions, therefore it is considered safe to execute the below code.
    unsafe {
      self.get_value_mut()[id] = item;
      self.wake_children();
    }
  }

  /// It retrieves the item located at `id`.
  ///
  /// `id`: The position for which the item needs to be returned.
  ///
  /// It returns the item located at `id` or `None` if `id` is out of bounds.
  pub fn get(&self, id: usize) -> Option<&T> {
    if id < self.get_value().len() {
      Some(&self.get_value()[id])
    } else {
      None
    }
  }

  /// It inserts a new `item` at the end of this array thereby increases the size of the array by 1.
  ///
  /// `item`: The new item to be inserted at the end of this array.
  pub fn push(&mut self, item: T) {
    // SAFETY: Changing the inner value before waking up all the child watchers does not violate the invariants of the
    // called functions, therefore it is considered safe to execute the below code.
    unsafe {
      self.get_value_mut().push(item);
      self.wake_children()
    }
  }

  /// It removes an old item at the end of this array thereby decreases the size of the array by 1.
  ///
  /// It returns the old item that is just removed from this array.
  pub fn pop(&mut self) -> Option<T> {
    // SAFETY: Changing the inner value before waking up all the child watchers does not violate the invariants of the
    // called functions, therefore it is considered safe to execute the below code.
    unsafe {
      let result = self.get_value_mut().pop();
      self.wake_children();
      result
    }
  }

  pub fn iter(&self) -> Iter<T> {
    Iter {
      id: 0,
      reactive: self,
    }
  }
}

impl<T: Debug> From<ReactiveSlice<T>> for Reactive<Vec<T>> {
  fn from(value: ReactiveSlice<T>) -> Self {
    value.proto.into()
  }
}

impl<T> IntoIterator for Reactive<Vec<T>> {
  type Item = T;
  type IntoIter = vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.into_value().into_iter()
  }
}

#[derive(Copy, Clone, Debug)]
pub struct Iter<'a, T> {
  id: usize,
  reactive: &'a Reactive<Vec<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.id != self.reactive.get_value().len() {
      let value = &self.reactive.get_value()[self.id];
      self.id += 1;
      Some(value)
    } else {
      None
    }
  }
}
