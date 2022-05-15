//! A trait that can make objects to become hashable into a slice of hash components called `Self::Part`.
//!
//! It is useful to allow storing or retrieving an array of objects into a
//! [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) where the key is part of objects
//! inside the array. Therefore, the array will be stored into the
//! [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) by each key component and the
//! [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) will contain that duplicate array as a
//! result.

use std::borrow::Cow;

/// A trait that can make objects to become hashable into a slice of hash components called `Self::Part`.
pub trait Hash
where
  [Self::Part]: ToOwned,
{
  /// The hash component type.
  type Part;

  /// Calculates a slice of hash components that can be used as a slice of keys for example.
  fn hash(&self) -> Cow<[Self::Part]>;
}
