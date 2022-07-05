use iron_ingot::TreeSet;
use std::borrow::Cow;

#[test]
fn test_bulk_put() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.bulk_put(
    [
      -23, -22, -21, -20, -19, -18, -17, -16, -15, -14, -13, -12, -11, -10, -9, -8, -7, -6, -5, -4,
      -3, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
      23,
    ]
    .into_iter(),
  );
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[
      &-23, &-22, &-21, &-20, &-19, &-18, &-17, &-16, &-15, &-14, &-13, &-12, &-11, &-10, &-9, &-8,
      &-7, &-6, &-5, &-4, &-3, &-2, &-1, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13,
      &14, &15, &16, &17, &18, &19, &20, &21, &22, &23,
    ])
  )
}

#[test]
fn test_put() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(0);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[&0]));
  tree_set.put(0);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[&0]));
  tree_set.put(-4);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[&-4, &0]));
  tree_set.put(4);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[&-4, &0, &4]));
  tree_set.put(-6);
  tree_set.put(-2);
  tree_set.put(2);
  tree_set.put(6);
  tree_set.put(-7);
  tree_set.put(-5);
  tree_set.put(-3);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(3);
  tree_set.put(5);
  tree_set.put(7);
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[&-7, &-6, &-5, &-4, &-3, &-2, &-1, &0, &1, &2, &3, &4, &5, &6, &7])
  );
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[
      &-7, &-6, &-5, &-4, &-3, &-2, &-1, &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12,
      &13, &14, &15, &16, &17, &18, &19, &20, &21, &22, &23
    ])
  );
  tree_set.put(-8);
  tree_set.put(-9);
  tree_set.put(-10);
  tree_set.put(-11);
  tree_set.put(-12);
  tree_set.put(-13);
  tree_set.put(-14);
  tree_set.put(-15);
  tree_set.put(-16);
  tree_set.put(-17);
  tree_set.put(-18);
  tree_set.put(-19);
  tree_set.put(-20);
  tree_set.put(-21);
  tree_set.put(-22);
  tree_set.put(-23);
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[
      &-23, &-22, &-21, &-20, &-19, &-18, &-17, &-16, &-15, &-14, &-13, &-12, &-11, &-10, &-9, &-8,
      &-7, &-6, &-5, &-4, &-3, &-2, &-1, &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12,
      &13, &14, &15, &16, &17, &18, &19, &20, &21, &22, &23
    ])
  );
  tree_set.put(-4);
  tree_set.put(4);
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[
      &-23, &-22, &-21, &-20, &-19, &-18, &-17, &-16, &-15, &-14, &-13, &-12, &-11, &-10, &-9, &-8,
      &-7, &-6, &-5, &-4, &-3, &-2, &-1, &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12,
      &13, &14, &15, &16, &17, &18, &19, &20, &21, &22, &23
    ])
  );
  tree_set.put(-23);
  tree_set.put(-22);
  tree_set.put(-21);
  tree_set.put(-20);
  tree_set.put(-19);
  tree_set.put(-18);
  tree_set.put(-17);
  tree_set.put(-16);
  tree_set.put(-15);
  tree_set.put(-14);
  tree_set.put(-13);
  tree_set.put(-12);
  tree_set.put(-11);
  tree_set.put(-10);
  tree_set.put(-9);
  tree_set.put(-8);
  tree_set.put(-7);
  tree_set.put(-6);
  tree_set.put(-5);
  tree_set.put(-4);
  tree_set.put(-3);
  tree_set.put(-2);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(2);
  tree_set.put(3);
  tree_set.put(4);
  tree_set.put(5);
  tree_set.put(6);
  tree_set.put(7);
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  assert_eq!(
    tree_set.to_slice(),
    Cow::Borrowed(&[
      &-23, &-22, &-21, &-20, &-19, &-18, &-17, &-16, &-15, &-14, &-13, &-12, &-11, &-10, &-9, &-8,
      &-7, &-6, &-5, &-4, &-3, &-2, &-1, &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12,
      &13, &14, &15, &16, &17, &18, &19, &20, &21, &22, &23
    ])
  );
}

#[test]
fn test_has() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(-23);
  tree_set.put(-22);
  tree_set.put(-21);
  tree_set.put(-20);
  tree_set.put(-19);
  tree_set.put(-18);
  tree_set.put(-17);
  tree_set.put(-16);
  tree_set.put(-15);
  tree_set.put(-14);
  tree_set.put(-13);
  tree_set.put(-12);
  tree_set.put(-11);
  tree_set.put(-10);
  tree_set.put(-9);
  tree_set.put(-8);
  tree_set.put(-7);
  tree_set.put(-6);
  tree_set.put(-5);
  tree_set.put(-4);
  tree_set.put(-3);
  tree_set.put(-2);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(2);
  tree_set.put(3);
  tree_set.put(4);
  tree_set.put(5);
  tree_set.put(6);
  tree_set.put(7);
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  assert!(!tree_set.has(-24));
  assert!(tree_set.has(-23));
  assert!(tree_set.has(-22));
  assert!(tree_set.has(-21));
  assert!(tree_set.has(-20));
  assert!(tree_set.has(-19));
  assert!(tree_set.has(-18));
  assert!(tree_set.has(-17));
  assert!(tree_set.has(-16));
  assert!(tree_set.has(-15));
  assert!(tree_set.has(-14));
  assert!(tree_set.has(-13));
  assert!(tree_set.has(-12));
  assert!(tree_set.has(-11));
  assert!(tree_set.has(-10));
  assert!(tree_set.has(-9));
  assert!(tree_set.has(-8));
  assert!(tree_set.has(-7));
  assert!(tree_set.has(-6));
  assert!(tree_set.has(-5));
  assert!(tree_set.has(-4));
  assert!(tree_set.has(-3));
  assert!(tree_set.has(-2));
  assert!(tree_set.has(-1));
  assert!(tree_set.has(1));
  assert!(tree_set.has(2));
  assert!(tree_set.has(3));
  assert!(tree_set.has(4));
  assert!(tree_set.has(5));
  assert!(tree_set.has(6));
  assert!(tree_set.has(7));
  assert!(tree_set.has(8));
  assert!(tree_set.has(9));
  assert!(tree_set.has(10));
  assert!(tree_set.has(11));
  assert!(tree_set.has(12));
  assert!(tree_set.has(13));
  assert!(tree_set.has(14));
  assert!(tree_set.has(15));
  assert!(tree_set.has(16));
  assert!(tree_set.has(17));
  assert!(tree_set.has(18));
  assert!(tree_set.has(19));
  assert!(tree_set.has(20));
  assert!(tree_set.has(21));
  assert!(tree_set.has(22));
  assert!(tree_set.has(23));
  assert!(!tree_set.has(-24));
}

#[test]
fn test_remove() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(-23);
  tree_set.put(-22);
  tree_set.put(-21);
  tree_set.put(-20);
  tree_set.put(-19);
  tree_set.put(-18);
  tree_set.put(-17);
  tree_set.put(-16);
  tree_set.put(-15);
  tree_set.put(-14);
  tree_set.put(-13);
  tree_set.put(-12);
  tree_set.put(-11);
  tree_set.put(-10);
  tree_set.put(-9);
  tree_set.put(-8);
  tree_set.put(-7);
  tree_set.put(-6);
  tree_set.put(-5);
  tree_set.put(-4);
  tree_set.put(-3);
  tree_set.put(-2);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(2);
  tree_set.put(3);
  tree_set.put(4);
  tree_set.put(5);
  tree_set.put(6);
  tree_set.put(7);
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  tree_set.remove(-24);
  tree_set.remove(-23);
  tree_set.remove(-22);
  tree_set.remove(-21);
  tree_set.remove(-20);
  tree_set.remove(-19);
  tree_set.remove(-18);
  tree_set.remove(-17);
  tree_set.remove(-16);
  tree_set.remove(-15);
  tree_set.remove(-14);
  tree_set.remove(-13);
  tree_set.remove(-12);
  tree_set.remove(-11);
  tree_set.remove(-10);
  tree_set.remove(-9);
  tree_set.remove(-8);
  tree_set.remove(-7);
  tree_set.remove(-6);
  tree_set.remove(-5);
  tree_set.remove(-4);
  tree_set.remove(-3);
  tree_set.remove(-2);
  tree_set.remove(-1);
  tree_set.remove(1);
  tree_set.remove(2);
  tree_set.remove(3);
  tree_set.remove(4);
  tree_set.remove(5);
  tree_set.remove(6);
  tree_set.remove(7);
  tree_set.remove(8);
  tree_set.remove(9);
  tree_set.remove(10);
  tree_set.remove(11);
  tree_set.remove(12);
  tree_set.remove(13);
  tree_set.remove(14);
  tree_set.remove(15);
  tree_set.remove(16);
  tree_set.remove(17);
  tree_set.remove(18);
  tree_set.remove(19);
  tree_set.remove(20);
  tree_set.remove(21);
  tree_set.remove(22);
  tree_set.remove(23);
  tree_set.remove(24);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[] as &[&i32]));
  tree_set.put(-23);
  tree_set.put(-22);
  tree_set.put(-21);
  tree_set.put(-20);
  tree_set.put(-19);
  tree_set.put(-18);
  tree_set.put(-17);
  tree_set.put(-16);
  tree_set.put(-15);
  tree_set.put(-14);
  tree_set.put(-13);
  tree_set.put(-12);
  tree_set.put(-11);
  tree_set.put(-10);
  tree_set.put(-9);
  tree_set.put(-8);
  tree_set.put(-7);
  tree_set.put(-6);
  tree_set.put(-5);
  tree_set.put(-4);
  tree_set.put(-3);
  tree_set.put(-2);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(2);
  tree_set.put(3);
  tree_set.put(4);
  tree_set.put(5);
  tree_set.put(6);
  tree_set.put(7);
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  tree_set.remove(24);
  tree_set.remove(23);
  tree_set.remove(22);
  tree_set.remove(21);
  tree_set.remove(20);
  tree_set.remove(19);
  tree_set.remove(18);
  tree_set.remove(17);
  tree_set.remove(16);
  tree_set.remove(15);
  tree_set.remove(14);
  tree_set.remove(13);
  tree_set.remove(12);
  tree_set.remove(11);
  tree_set.remove(10);
  tree_set.remove(9);
  tree_set.remove(8);
  tree_set.remove(7);
  tree_set.remove(6);
  tree_set.remove(5);
  tree_set.remove(4);
  tree_set.remove(3);
  tree_set.remove(2);
  tree_set.remove(1);
  tree_set.remove(-1);
  tree_set.remove(-2);
  tree_set.remove(-3);
  tree_set.remove(-4);
  tree_set.remove(-5);
  tree_set.remove(-6);
  tree_set.remove(-7);
  tree_set.remove(-8);
  tree_set.remove(-9);
  tree_set.remove(-10);
  tree_set.remove(-11);
  tree_set.remove(-12);
  tree_set.remove(-13);
  tree_set.remove(-14);
  tree_set.remove(-15);
  tree_set.remove(-16);
  tree_set.remove(-17);
  tree_set.remove(-18);
  tree_set.remove(-19);
  tree_set.remove(-20);
  tree_set.remove(-21);
  tree_set.remove(-22);
  tree_set.remove(-23);
  tree_set.remove(-24);
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[] as &[&i32]));
}

#[test]
fn test_is_empty() {
  let mut tree_set = TreeSet::<i32>::new(());
  assert!(tree_set.is_empty());
  tree_set.put(0);
  assert!(!tree_set.is_empty());
  tree_set.put(0);
  assert!(!tree_set.is_empty());
  tree_set.put(-4);
  assert!(!tree_set.is_empty());
  tree_set.put(4);
  assert!(!tree_set.is_empty());
  tree_set.put(-6);
  assert!(!tree_set.is_empty());
}

#[test]
fn test_len() {
  let mut tree_set = TreeSet::<i32>::new(());
  assert_eq!(tree_set.len(), 0);
  tree_set.put(0);
  assert_eq!(tree_set.len(), 1);
  tree_set.put(0);
  assert_eq!(tree_set.len(), 1);
  tree_set.put(-4);
  assert_eq!(tree_set.len(), 2);
  tree_set.put(4);
  assert_eq!(tree_set.len(), 3);
  tree_set.put(-6);
  assert_eq!(tree_set.len(), 4);
}

#[test]
fn test_min() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(0);
  assert_eq!(tree_set.min(), Some(&0));
  tree_set.put(0);
  assert_eq!(tree_set.min(), Some(&0));
  tree_set.put(-4);
  assert_eq!(tree_set.min(), Some(&-4));
  tree_set.put(4);
  assert_eq!(tree_set.min(), Some(&-4));
  tree_set.put(-6);
  assert_eq!(tree_set.min(), Some(&-6));
  tree_set.put(-2);
  assert_eq!(tree_set.min(), Some(&-6));
  tree_set.put(2);
  assert_eq!(tree_set.min(), Some(&-6));
  tree_set.put(6);
  assert_eq!(tree_set.min(), Some(&-6));
  tree_set.put(-7);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(-5);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(-3);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(-1);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(1);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(3);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(5);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(7);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(8);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(9);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(10);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(11);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(12);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(13);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(14);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(15);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(16);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(17);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(18);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(19);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(20);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(21);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(22);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(23);
  assert_eq!(tree_set.min(), Some(&-7));
  tree_set.put(-8);
  assert_eq!(tree_set.min(), Some(&-8));
  tree_set.put(-9);
  assert_eq!(tree_set.min(), Some(&-9));
  tree_set.put(-10);
  assert_eq!(tree_set.min(), Some(&-10));
  tree_set.put(-11);
  assert_eq!(tree_set.min(), Some(&-11));
  tree_set.put(-12);
  assert_eq!(tree_set.min(), Some(&-12));
  tree_set.put(-13);
  assert_eq!(tree_set.min(), Some(&-13));
  tree_set.put(-14);
  assert_eq!(tree_set.min(), Some(&-14));
  tree_set.put(-15);
  assert_eq!(tree_set.min(), Some(&-15));
  tree_set.put(-16);
  assert_eq!(tree_set.min(), Some(&-16));
  tree_set.put(-17);
  assert_eq!(tree_set.min(), Some(&-17));
  tree_set.put(-18);
  assert_eq!(tree_set.min(), Some(&-18));
  tree_set.put(-19);
  assert_eq!(tree_set.min(), Some(&-19));
  tree_set.put(-20);
  assert_eq!(tree_set.min(), Some(&-20));
  tree_set.put(-21);
  assert_eq!(tree_set.min(), Some(&-21));
  tree_set.put(-22);
  assert_eq!(tree_set.min(), Some(&-22));
  tree_set.put(-23);
  assert_eq!(tree_set.min(), Some(&-23));
  tree_set.put(-4);
  assert_eq!(tree_set.min(), Some(&-23));
  tree_set.put(4);
  assert_eq!(tree_set.min(), Some(&-23));
}

#[test]
fn test_max() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(0);
  assert_eq!(tree_set.max(), Some(&0));
  tree_set.put(0);
  assert_eq!(tree_set.max(), Some(&0));
  tree_set.put(-4);
  assert_eq!(tree_set.max(), Some(&0));
  tree_set.put(4);
  assert_eq!(tree_set.max(), Some(&4));
  tree_set.put(-6);
  assert_eq!(tree_set.max(), Some(&4));
  tree_set.put(-2);
  assert_eq!(tree_set.max(), Some(&4));
  tree_set.put(2);
  assert_eq!(tree_set.max(), Some(&4));
  tree_set.put(6);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(-7);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(-5);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(-3);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(-1);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(1);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(3);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(5);
  assert_eq!(tree_set.max(), Some(&6));
  tree_set.put(7);
  assert_eq!(tree_set.max(), Some(&7));
  tree_set.put(8);
  assert_eq!(tree_set.max(), Some(&8));
  tree_set.put(9);
  assert_eq!(tree_set.max(), Some(&9));
  tree_set.put(10);
  assert_eq!(tree_set.max(), Some(&10));
  tree_set.put(11);
  assert_eq!(tree_set.max(), Some(&11));
  tree_set.put(12);
  assert_eq!(tree_set.max(), Some(&12));
  tree_set.put(13);
  assert_eq!(tree_set.max(), Some(&13));
  tree_set.put(14);
  assert_eq!(tree_set.max(), Some(&14));
  tree_set.put(15);
  assert_eq!(tree_set.max(), Some(&15));
  tree_set.put(16);
  assert_eq!(tree_set.max(), Some(&16));
  tree_set.put(17);
  assert_eq!(tree_set.max(), Some(&17));
  tree_set.put(18);
  assert_eq!(tree_set.max(), Some(&18));
  tree_set.put(19);
  assert_eq!(tree_set.max(), Some(&19));
  tree_set.put(20);
  assert_eq!(tree_set.max(), Some(&20));
  tree_set.put(21);
  assert_eq!(tree_set.max(), Some(&21));
  tree_set.put(22);
  assert_eq!(tree_set.max(), Some(&22));
  tree_set.put(23);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-8);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-9);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-10);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-11);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-12);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-13);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-14);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-15);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-16);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-17);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-18);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-19);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-20);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-21);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-22);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-23);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(-4);
  assert_eq!(tree_set.max(), Some(&23));
  tree_set.put(4);
  assert_eq!(tree_set.max(), Some(&23));
}

#[test]
fn test_to_slice() {
  assert_eq!(
    TreeSet::<i32>::from([].into_iter()).to_slice(),
    Cow::Borrowed(&[] as &[&i32])
  );
  assert_eq!(
    TreeSet::from([1].into_iter()).to_slice(),
    Cow::Borrowed(&[&1])
  );
  assert_eq!(
    TreeSet::from([1, 2].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3, 4].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3, &4])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3, 4, 5].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3, &4, &5])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3, 4, 5, 6].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3, &4, &5, &6,])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3, 4, 5, 6, 7,].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3, &4, &5, &6, &7,])
  );
  assert_eq!(
    TreeSet::from([1, 2, 3, 4, 5, 6, 7, 8,].into_iter()).to_slice(),
    Cow::Borrowed(&[&1, &2, &3, &4, &5, &6, &7, &8,])
  );
}

#[test]
fn test_clear() {
  let mut tree_set = TreeSet::<i32>::new(());
  tree_set.put(-23);
  tree_set.put(-22);
  tree_set.put(-21);
  tree_set.put(-20);
  tree_set.put(-19);
  tree_set.put(-18);
  tree_set.put(-17);
  tree_set.put(-16);
  tree_set.put(-15);
  tree_set.put(-14);
  tree_set.put(-13);
  tree_set.put(-12);
  tree_set.put(-11);
  tree_set.put(-10);
  tree_set.put(-9);
  tree_set.put(-8);
  tree_set.put(-7);
  tree_set.put(-6);
  tree_set.put(-5);
  tree_set.put(-4);
  tree_set.put(-3);
  tree_set.put(-2);
  tree_set.put(-1);
  tree_set.put(1);
  tree_set.put(2);
  tree_set.put(3);
  tree_set.put(4);
  tree_set.put(5);
  tree_set.put(6);
  tree_set.put(7);
  tree_set.put(8);
  tree_set.put(9);
  tree_set.put(10);
  tree_set.put(11);
  tree_set.put(12);
  tree_set.put(13);
  tree_set.put(14);
  tree_set.put(15);
  tree_set.put(16);
  tree_set.put(17);
  tree_set.put(18);
  tree_set.put(19);
  tree_set.put(20);
  tree_set.put(21);
  tree_set.put(22);
  tree_set.put(23);
  tree_set.clear();
  assert_eq!(tree_set.to_slice(), Cow::Borrowed(&[] as &[&i32]));
}
