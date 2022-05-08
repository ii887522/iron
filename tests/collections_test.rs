extern crate iron;

use iron::collections::add;
use iron::collections::into_map;
use iron::collections::into_slice;
use iron::collections::max;
use iron::collections::min;
use iron::collections::subtract;
use iron::Hash;
use std::borrow::Cow;
use std::collections::HashMap;

#[test]
fn test_min() {
  assert_eq!(&*min(&[], |_: &i32, _| 0), &[] as &[&i32]);
  assert_eq!(&*min(&[0], |_, _| 0), &[&0]);
  assert_eq!(&*min(&[1], |_, _| 0), &[&1]);
  assert_eq!(&*min(&[2], |_, _| 0), &[&2]);
  assert_eq!(&*min(&[2, 0], |_, _| 0), &[&2, &0]);
  assert_eq!(&*min(&[2, 1], |_, _| 0), &[&2, &1]);
  assert_eq!(&*min(&[3, 1], |_, _| 0), &[&3, &1]);
  assert_eq!(&*min(&[3, 1, 0], |_, _| 0), &[&3, &1, &0]);
  assert_eq!(&*min(&[3, 1, 0], |&value, _| value), &[&0]);
  assert_eq!(&*min(&[3, 1, 0], |&value, _| -value), &[&3]);
}

#[test]
fn test_max() {
  assert_eq!(&*max(&[], |_: &i32, _| 0), &[] as &[&i32]);
  assert_eq!(&*max(&[0], |_, _| 0), &[&0]);
  assert_eq!(&*max(&[1], |_, _| 0), &[&1]);
  assert_eq!(&*max(&[2], |_, _| 0), &[&2]);
  assert_eq!(&*max(&[2, 0], |_, _| 0), &[&2, &0]);
  assert_eq!(&*max(&[2, 1], |_, _| 0), &[&2, &1]);
  assert_eq!(&*max(&[3, 1], |_, _| 0), &[&3, &1]);
  assert_eq!(&*max(&[3, 1, 0], |_, _| 0), &[&3, &1, &0]);
  assert_eq!(&*max(&[3, 1, 0], |&value, _| value), &[&3]);
  assert_eq!(&*max(&[3, 1, 0], |&value, _| -value), &[&0]);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Value<'a>(i32, &'a [&'a usize]);

impl<'a> Hash for Value<'a> {
  type Part = &'a usize;

  fn hash(&self) -> Cow<[Self::Part]> {
    self.1.into()
  }
}

#[test]
fn test_into_slice() {
  assert_eq!(
    &*into_slice::<usize, Value>(HashMap::new()),
    &[] as &[&Value]
  );
  assert_eq!(
    &*into_slice(HashMap::from([(&0, &[&Value(0, &[&0])] as &[&Value])])),
    &[&Value(0, &[&0])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([(&0, &[&Value(1, &[&0])] as &[&Value])])),
    &[&Value(1, &[&0])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([(&0, &[&Value(2, &[&0])] as &[&Value])])),
    &[&Value(2, &[&0])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([(&1, &[&Value(2, &[&1])] as &[&Value])])),
    &[&Value(2, &[&1])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([(&2, &[&Value(2, &[&2])] as &[&Value])])),
    &[&Value(2, &[&2])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([
      (&0, &[&Value(2, &[&2, &0])] as &[&Value]),
      (&2, &[&Value(2, &[&2, &0])])
    ])),
    &[&Value(2, &[&2, &0])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([
      (&1, &[&Value(2, &[&2, &1])] as &[&Value]),
      (&2, &[&Value(2, &[&2, &1])])
    ])),
    &[&Value(2, &[&2, &1])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([
      (&1, &[&Value(2, &[&3, &1])] as &[&Value]),
      (&3, &[&Value(2, &[&3, &1])])
    ])),
    &[&Value(2, &[&3, &1])]
  );
  assert_eq!(
    &*into_slice(HashMap::from([
      (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
      (&1, &[&Value(2, &[&3, &1, &0])]),
      (&3, &[&Value(2, &[&3, &1, &0])])
    ])),
    &[&Value(2, &[&3, &1, &0])]
  );
  {
    let mut array = into_slice(HashMap::from([
      (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
      (&1, &[&Value(2, &[&3, &1, &0])]),
      (&2, &[&Value(0, &[&2])]),
      (&3, &[&Value(2, &[&3, &1, &0])]),
    ]))
    .into_owned();
    array.sort();
    assert_eq!(array, vec![&Value(0, &[&2]), &Value(2, &[&3, &1, &0])]);
  }
  {
    let mut array = into_slice(HashMap::from([
      (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
      (&1, &[&Value(2, &[&3, &1, &0])]),
      (&2, &[&Value(1, &[&2])]),
      (&3, &[&Value(2, &[&3, &1, &0])]),
    ]))
    .into_owned();
    array.sort();
    assert_eq!(array, vec![&Value(1, &[&2]), &Value(2, &[&3, &1, &0]),]);
  }
  {
    let mut array = into_slice(HashMap::from([
      (&0, &[&Value(3, &[&3, &1, &0])] as &[&Value]),
      (&1, &[&Value(3, &[&3, &1, &0])]),
      (&2, &[&Value(1, &[&2])]),
      (&3, &[&Value(3, &[&3, &1, &0])]),
    ]))
    .into_owned();
    array.sort();
    assert_eq!(array, vec![&Value(1, &[&2]), &Value(3, &[&3, &1, &0])]);
  }
  {
    let mut array = into_slice(HashMap::from([
      (&0, &[&Value(3, &[&3, &1, &0])] as &[&Value]),
      (&1, &[&Value(3, &[&3, &1, &0])]),
      (&2, &[&Value(1, &[&2])]),
      (&3, &[&Value(3, &[&3, &1, &0])]),
      (&4, &[&Value(0, &[&4])]),
    ]))
    .into_owned();
    array.sort();
    assert_eq!(
      array,
      vec![&Value(0, &[&4]), &Value(1, &[&2]), &Value(3, &[&3, &1, &0])]
    )
  }
}

#[test]
fn test_map() {
  assert_eq!(
    into_map::<usize, Value>(&[]),
    HashMap::<&usize, Cow<[&Value]>>::from([])
  );
  assert_eq!(
    into_map(&[&Value(0, &[&0])]),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(0, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    into_map(&[&Value(1, &[&0])]),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(1, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&0])]),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(2, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&1])]),
    HashMap::from([(&1, Cow::Borrowed(&[&Value(2, &[&1])] as &[&Value]))])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&2])]),
    HashMap::from([(&2, Cow::Borrowed(&[&Value(2, &[&2])] as &[&Value]))])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&2, &0])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&2, &0])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &0])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&2, &1])]),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&2, &1])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &1])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&3, &1])]),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1])] as &[&Value])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&3, &1, &0])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&3, &1, &0]), &Value(0, &[&2])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&2, Cow::Borrowed(&[&Value(0, &[&2])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(2, &[&3, &1, &0]), &Value(1, &[&2])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&2, Cow::Borrowed(&[&Value(1, &[&2])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(3, &[&3, &1, &0]), &Value(1, &[&2])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])])),
      (&2, Cow::Borrowed(&[&Value(1, &[&2])])),
      (&3, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    into_map(&[&Value(3, &[&3, &1, &0]), &Value(1, &[&2]), &Value(0, &[&4])]),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])])),
      (&2, Cow::Borrowed(&[&Value(1, &[&2])])),
      (&3, Cow::Borrowed(&[&Value(3, &[&3, &1, &0])])),
      (&4, Cow::Borrowed(&[&Value(0, &[&4])]))
    ])
  );
}

#[test]
fn test_add() {
  assert_eq!(
    add::<usize, Value>(HashMap::new(), HashMap::new()),
    HashMap::<&usize, Cow<[&Value]>>::new()
  );
  assert_eq!(
    add(
      HashMap::from([(&0, &[&Value(0, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(0, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    add(
      HashMap::from([(&0, &[&Value(1, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(1, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    add(
      HashMap::from([(&0, &[&Value(2, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(2, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    add(
      HashMap::from([(&1, &[&Value(2, &[&1])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&1, Cow::Borrowed(&[&Value(2, &[&1])] as &[&Value]))])
  );
  assert_eq!(
    add(
      HashMap::from([(&2, &[&Value(2, &[&2])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&2, Cow::Borrowed(&[&Value(2, &[&2])] as &[&Value]))])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&2, &0])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &0])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&2, &0])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&1, &[&Value(2, &[&2, &1])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &1])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&2, &1])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &1])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&1, &[&Value(2, &[&3, &1])] as &[&Value]),
        (&3, &[&Value(2, &[&3, &1])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1])] as &[&Value])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(0, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (
        &0,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(0, &[&0])] as &[&Value])
      ),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(1, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (
        &0,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(1, &[&0])] as &[&Value])
      ),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(2, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (
        &0,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(2, &[&0])] as &[&Value])
      ),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&1, &[&Value(2, &[&1])] as &[&Value])])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (
        &1,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(2, &[&1])])
      ),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&1, &[&Value(2, &[&2, &1])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &1])])
      ])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (
        &1,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(2, &[&2, &1])])
      ),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &1])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&1, &[&Value(2, &[&3, &1])] as &[&Value]),
        (&3, &[&Value(2, &[&3, &1])])
      ])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (
        &1,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(2, &[&3, &1])])
      ),
      (
        &3,
        Cow::Borrowed(&[&Value(2, &[&3, &1, &0]), &Value(2, &[&3, &1])])
      )
    ])
  );
  assert_eq!(
    add(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
}

#[test]
fn test_subtract() {
  assert_eq!(
    subtract::<usize, Value>(HashMap::new(), HashMap::new()),
    HashMap::<&usize, Cow<[&Value]>>::new()
  );
  assert_eq!(
    subtract(
      HashMap::from([(&0, &[&Value(0, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(0, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    subtract(
      HashMap::from([(&0, &[&Value(1, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(1, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    subtract(
      HashMap::from([(&0, &[&Value(2, &[&0])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&0, Cow::Borrowed(&[&Value(2, &[&0])] as &[&Value]))])
  );
  assert_eq!(
    subtract(
      HashMap::from([(&1, &[&Value(2, &[&1])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&1, Cow::Borrowed(&[&Value(2, &[&1])] as &[&Value]))])
  );
  assert_eq!(
    subtract(
      HashMap::from([(&2, &[&Value(2, &[&2])] as &[&Value])]),
      HashMap::new()
    ),
    HashMap::from([(&2, Cow::Borrowed(&[&Value(2, &[&2])] as &[&Value]))])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&2, &0])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &0])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&2, &0])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&1, &[&Value(2, &[&2, &1])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &1])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&2, &1])] as &[&Value])),
      (&2, Cow::Borrowed(&[&Value(2, &[&2, &1])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&1, &[&Value(2, &[&3, &1])] as &[&Value]),
        (&3, &[&Value(2, &[&3, &1])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1])] as &[&Value])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::new()
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(0, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(1, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&0, &[&Value(2, &[&0])] as &[&Value])])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([(&1, &[&Value(2, &[&1])] as &[&Value])])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&1, &[&Value(2, &[&2, &1])] as &[&Value]),
        (&2, &[&Value(2, &[&2, &1])])
      ])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&1, &[&Value(2, &[&3, &1])] as &[&Value]),
        (&3, &[&Value(2, &[&3, &1])])
      ])
    ),
    HashMap::from([
      (&0, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])] as &[&Value])),
      (&1, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])])),
      (&3, Cow::Borrowed(&[&Value(2, &[&3, &1, &0])]))
    ])
  );
  assert_eq!(
    subtract(
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ]),
      HashMap::from([
        (&0, &[&Value(2, &[&3, &1, &0])] as &[&Value]),
        (&1, &[&Value(2, &[&3, &1, &0])]),
        (&3, &[&Value(2, &[&3, &1, &0])])
      ])
    ),
    HashMap::new()
  );
}
