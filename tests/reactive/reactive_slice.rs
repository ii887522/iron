use iron::{Reactive, ReactiveSlice};

#[test]
fn test_set() {
  let array = ReactiveSlice::new(vec![0, 0, 0]);
  array.borrow_mut().set(0, 1);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&1, &0, &0]);
  array.borrow_mut().set(0, 2);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&2, &0, &0]);
  array.borrow_mut().set(0, 3);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&3, &0, &0]);
  array.borrow_mut().set(1, 3);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&3, &3, &0]);
  array.borrow_mut().set(2, 3);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&3, &3, &3]);
}

#[test]
fn test_push() {
  let array = ReactiveSlice::new(());
  array.borrow_mut().push(0);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0]);
  array.borrow_mut().push(1);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0, &1]);
  array.borrow_mut().push(2);
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0, &1, &2]);
}

#[test]
fn test_pop() {
  let array = ReactiveSlice::new(vec![0, 1, 2, 3]);
  assert_eq!(array.borrow_mut().pop(), Some(3));
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0, &1, &2]);
  assert_eq!(array.borrow_mut().pop(), Some(2));
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0, &1]);
  assert_eq!(array.borrow_mut().pop(), Some(1));
  assert_eq!(array.borrow().iter().collect::<Vec<_>>(), vec![&0]);
  assert_eq!(array.borrow_mut().pop(), Some(0));
  assert_eq!(
    array.borrow().iter().collect::<Vec<_>>(),
    Vec::<&i32>::new()
  );
  assert_eq!(array.borrow_mut().pop(), None);
  assert_eq!(
    array.borrow().iter().collect::<Vec<_>>(),
    Vec::<&i32>::new()
  );
}

#[test]
fn test_iter() {
  assert_eq!(
    ReactiveSlice::<i32>::new(vec![])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    Vec::<&i32>::new()
  );
  assert_eq!(
    ReactiveSlice::new(vec![0])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&0]
  );
  assert_eq!(
    ReactiveSlice::new(vec![1])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&1]
  );
  assert_eq!(
    ReactiveSlice::new(vec![2])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&2]
  );
  assert_eq!(
    ReactiveSlice::new(vec![2, 1])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&2, &1]
  );
  assert_eq!(
    ReactiveSlice::new(vec![3, 1])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&3, &1]
  );
  assert_eq!(
    ReactiveSlice::new(vec![3, 1, 0])
      .borrow()
      .iter()
      .collect::<Vec<_>>(),
    vec![&3, &1, &0]
  );
}

#[test]
fn test_into_iter() {
  assert_eq!(
    Reactive::from(ReactiveSlice::<i32>::new(vec![]))
      .into_iter()
      .collect::<Vec<_>>(),
    Vec::<i32>::new()
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![0]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![0]
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![1]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![1]
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![2]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![2]
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![2, 1]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![2, 1]
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![3, 1]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![3, 1]
  );
  assert_eq!(
    Reactive::from(ReactiveSlice::new(vec![3, 1, 0]))
      .into_iter()
      .collect::<Vec<_>>(),
    vec![3, 1, 0]
  );
}
