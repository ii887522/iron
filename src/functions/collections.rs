use std::{
  borrow::Cow,
  cmp::Ordering::{Equal, Greater, Less},
  collections::HashMap,
  hash::Hash,
};

/// It find the minimum objects from the `slice` received determined by the value retrieved from each object in the
/// `slice` through the `get_value` function given.
///
/// `slice`: The slice to be searched from.
///
/// `get_value`: The function that retrieves a value from the object in the `slice`.
///
/// It returns the minimum objects.
pub fn min<T, R: Ord>(array: &[T], mut get_value: impl FnMut(&T, usize) -> R) -> Cow<[&T]> {
  let mut min_value: Option<R> = None;
  let mut result = vec![];
  for (i, any) in array.iter().enumerate() {
    let value = get_value(any, i);
    if let Some(unwrapped_min_value) = &min_value {
      match value.cmp(unwrapped_min_value) {
        Greater => continue,
        Less => {
          min_value = Some(value);
          result = vec![any];
        }
        Equal => result.push(any),
      }
    } else {
      min_value = Some(value);
      result = vec![any];
    }
  }
  result.into()
}

/// It find the maximum objects from the `slice` received determined by the value retrieved from each object in the
/// `slice` through the `get_value` function given.
///
/// `slice`: The array to be searched from.
///
/// `get_value: The function that retrieves a value from the object in the `slice`.
///
/// It returns the maximum objects.
pub fn max<T, R: Ord>(array: &[T], mut get_value: impl FnMut(&T, usize) -> R) -> Cow<[&T]> {
  let mut max_value: Option<R> = None;
  let mut result = vec![];
  for (i, any) in array.iter().enumerate() {
    let value = get_value(any, i);
    if let Some(unwrapped_max_value) = &max_value {
      match value.cmp(unwrapped_max_value) {
        Less => continue,
        Greater => {
          max_value = Some(value);
          result = vec![any];
        }
        Equal => result.push(any),
      }
    } else {
      max_value = Some(value);
      result = vec![any];
    }
  }
  result.into()
}

/// It converts the `map` given which may have duplicate objects into a slice without duplicates.
///
/// `map`: The map to be converted from.
///
/// It returns a slice without duplicates.
pub fn into_slice<'a, K: Hash + Eq, V: crate::Hash<Part = &'a K>>(
  map: HashMap<&'a K, &'a [&V]>,
) -> Cow<'a, [&'a V]> {
  let mut result = Vec::with_capacity(map.len());
  let mut is_exists = HashMap::with_capacity(map.len());
  for &anys in map.values() {
    for &any in anys {
      if is_exists.get(&any.hash()).is_some() {
        continue;
      }
      result.push(any);
      is_exists.entry(any.hash()).or_insert(true);
    }
  }
  result.into()
}

/// It converts the `slice` given without duplicate objects into a map with duplicates.
///
/// `slice`: The slice to be converted from.
///
/// It returns a map with duplicates.
pub fn into_map<'a, K: Hash + Eq, V: crate::Hash<Part = &'a K>>(
  slice: &'a [&V],
) -> HashMap<&'a K, Cow<'a, [&'a V]>> {
  let mut result = HashMap::<_, Vec<_>>::new();
  for &any in slice {
    for &hash_part in &*any.hash() {
      if let Some(row) = result.get_mut(hash_part) {
        row.push(any);
      } else {
        result.entry(hash_part).or_insert_with(|| vec![any]);
      }
    }
  }
  result
    .into_iter()
    .map(|(key, value)| (key, value.into()))
    .collect()
}

/// It concatenates two maps received that may have duplicate objects into a map which contains duplicates.
///
/// `am`: The first map to include.
///
/// `bm`: The second map to include.
///
/// It returns a map which contains duplicates.
pub fn add<'a, K: Hash + Eq, V: crate::Hash<Part = &'a K>>(
  am: HashMap<&'a K, &'a [&V]>,
  bm: HashMap<&'a K, &'a [&V]>,
) -> HashMap<&'a K, Cow<'a, [&'a V]>> {
  let mut rm = HashMap::<_, Vec<_>>::with_capacity(am.len() + bm.len());
  let mut is_exists = HashMap::with_capacity(am.len());
  for &a in into_slice(am).iter() {
    for &hash_part in a.hash().iter() {
      if let Some(rr) = rm.get_mut(hash_part) {
        rr.push(a);
      } else {
        rm.entry(hash_part).or_insert_with(|| vec![a]);
      }
    }
    is_exists.entry(a.hash()).or_insert(true);
  }
  for &b in into_slice(bm).iter() {
    if is_exists.get(&b.hash()).is_some() {
      continue;
    }
    for &hash_part in b.hash().iter() {
      if let Some(rr) = rm.get_mut(hash_part) {
        rr.push(b);
      } else {
        rm.entry(hash_part).or_insert_with(|| vec![b]);
      }
    }
  }
  rm.into_iter()
    .map(|(key, value)| (key, value.into()))
    .collect()
}

/// It removes objects from the first map where they exist in the second map and returns it.
///
/// `am`: The map to remove objects from.
///
/// `bm`: The map which contains the objects to be searched from the first map for removal.
///
/// It returns the first map without objects that exist in the second map.
pub fn sub<'a, K: Hash + Eq, V: crate::Hash<Part = &'a K>>(
  am: HashMap<&'a K, &'a [&V]>,
  bm: HashMap<&'a K, &'a [&V]>,
) -> HashMap<&'a K, Cow<'a, [&'a V]>> {
  let mut rm = HashMap::<_, Vec<_>>::with_capacity(am.len());
  let mut is_exists = HashMap::with_capacity(bm.len());
  for &b in into_slice(bm).iter() {
    is_exists.entry(b.hash()).or_insert(true);
  }
  for &a in into_slice(am).iter() {
    if is_exists.get(&a.hash()).is_some() {
      continue;
    }
    for &hash_part in a.hash().iter() {
      if let Some(rr) = rm.get_mut(hash_part) {
        rr.push(a);
      } else {
        rm.entry(hash_part).or_insert_with(|| vec![a]);
      }
    }
  }
  rm.into_iter()
    .map(|(key, value)| (key, value.into()))
    .collect()
}
