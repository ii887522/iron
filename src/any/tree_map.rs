use std::{
  borrow::Cow,
  ops::{Index, IndexMut},
};

#[derive(Copy, Clone, Debug)]
pub struct Arg(usize);

impl From<()> for Arg {
  fn from(_: ()) -> Self {
    Self(32)
  }
}

impl From<usize> for Arg {
  fn from(initial_capacity: usize) -> Self {
    Self(initial_capacity)
  }
}

#[derive(Debug, Default)]
pub struct TreeMap<K, V> {
  nodes: Vec<Option<Node<K, V>>>,
  hole_ids: Vec<usize>,
  root: usize,
}

impl<K: Default + PartialOrd, V: Default, Iter: Iterator<Item = (K, V)>> From<Iter>
  for TreeMap<K, V>
{
  fn from(iter: Iter) -> Self {
    let mut this = Self::new(());
    this.bulk_put(iter);
    this
  }
}

impl<K, V> TreeMap<K, V> {
  pub fn new(arg: impl Into<Arg>) -> Self {
    let Arg(initial_capacity) = arg.into();
    let mut nodes = Vec::with_capacity(initial_capacity + 1);
    nodes.push(None);
    Self {
      nodes,
      hole_ids: Vec::with_capacity(initial_capacity),
      root: 0,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn len(&self) -> usize {
    self.nodes.len() - 1 - self.hole_ids.len()
  }

  fn update(&mut self, id: usize) {
    let mut cursor = id;
    while cursor != 0 {
      self.update_heights(cursor);
      if self.is_biased(cursor) {
        self.balance(cursor);
      }
      cursor = self.nodes[cursor].as_ref().unwrap().parent;
    }
  }

  fn update_heights(&mut self, id: usize) {
    self.update_left_height(id);
    self.update_right_height(id);
  }

  fn update_left_height(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let left = node.left;
    self.nodes[id].as_mut().unwrap().left_height = if left == 0 {
      0
    } else {
      let left_node = self.nodes[left].as_ref().unwrap();
      left_node.left_height.max(left_node.right_height) + 1
    }
  }

  fn update_right_height(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let right = node.right;
    self.nodes[id].as_mut().unwrap().right_height = if right == 0 {
      0
    } else {
      let right_node = self.nodes[right].as_ref().unwrap();
      right_node.left_height.max(right_node.right_height) + 1
    }
  }

  fn is_biased(&self, id: usize) -> bool {
    let node = self.nodes[id].as_ref().unwrap();
    node.left_height.abs_diff(node.right_height) > 1
  }

  fn balance(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    if node.left_height < node.right_height {
      let right_node = self.nodes[node.right].as_ref().unwrap();
      if right_node.left_height < right_node.right_height {
        self.rotate_left(id);
      } else {
        self.flip_right(id);
      }
    } else {
      let left_node = self.nodes[node.left].as_ref().unwrap();
      if left_node.left_height < left_node.right_height {
        self.flip_left(id);
      } else {
        self.rotate_right(id);
      }
    }
  }

  fn rotate_left(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let p = node.parent;
    let r = node.right;
    let rl = self.nodes[r].as_ref().unwrap().left;
    if p == 0 {
      self.root = r;
    } else {
      let p_node = self.nodes[p].as_mut().unwrap();
      if p_node.left == id {
        p_node.left = r;
      } else {
        p_node.right = r;
      }
    }
    let node = self.nodes[id].as_mut().unwrap();
    node.right = rl;
    node.parent = r;
    let r_node = self.nodes[r].as_mut().unwrap();
    r_node.left = id;
    r_node.parent = p;
    if let Some(node) = self.nodes[rl].as_mut() {
      node.parent = id;
    }
    self.update_right_height(id);
  }

  fn flip_right(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let p = node.parent;
    let r = node.right;
    let rl = self.nodes[r].as_ref().unwrap().left;
    let rl_node = self.nodes[rl].as_ref().unwrap();
    let rll = rl_node.left;
    let rlr = rl_node.right;
    if p == 0 {
      self.root = rl;
    } else {
      let p_node = self.nodes[p].as_mut().unwrap();
      if p_node.left == id {
        p_node.left = rl;
      } else {
        p_node.right = rl;
      }
    }
    let r_node = self.nodes[r].as_mut().unwrap();
    r_node.parent = rl;
    r_node.left = rlr;
    let rl_node = self.nodes[rl].as_mut().unwrap();
    rl_node.parent = p;
    rl_node.left = id;
    rl_node.right = r;
    let node = self.nodes[id].as_mut().unwrap();
    node.parent = rl;
    node.right = rll;
    if let Some(node) = self.nodes[rll].as_mut() {
      node.parent = id;
    }
    if let Some(node) = self.nodes[rlr].as_mut() {
      node.parent = r;
    }
    self.update_right_height(id);
    self.update_left_height(r);
  }

  fn rotate_right(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let p = node.parent;
    let l = node.left;
    let lr = self.nodes[l].as_ref().unwrap().right;
    if p == 0 {
      self.root = l;
    } else {
      let p_node = self.nodes[p].as_mut().unwrap();
      if p_node.left == id {
        p_node.left = l;
      } else {
        p_node.right = l;
      }
    }
    let node = self.nodes[id].as_mut().unwrap();
    node.parent = l;
    node.left = lr;
    let l_node = self.nodes[l].as_mut().unwrap();
    l_node.right = id;
    l_node.parent = p;
    if let Some(node) = self.nodes[lr].as_mut() {
      node.parent = id;
    }
    self.update_left_height(id);
  }

  fn flip_left(&mut self, id: usize) {
    let node = self.nodes[id].as_ref().unwrap();
    let p = node.parent;
    let l = node.left;
    let lr = self.nodes[l].as_ref().unwrap().right;
    let lr_node = self.nodes[lr].as_mut().unwrap();
    let lrr = lr_node.right;
    let lrl = lr_node.left;
    let p_node = self.nodes[p].as_mut().unwrap();
    if p == 0 {
      self.root = lr;
    } else if p_node.left == id {
      p_node.left = lr;
    } else {
      p_node.right = lr;
    }
    let l_node = self.nodes[l].as_mut().unwrap();
    l_node.parent = lr;
    l_node.right = lrl;
    let lr_node = self.nodes[lr].as_mut().unwrap();
    lr_node.parent = p;
    lr_node.right = id;
    lr_node.left = l;
    let node = self.nodes[id].as_mut().unwrap();
    node.parent = lr;
    node.left = lrr;
    self.nodes[lrr].as_mut().unwrap().parent = id;
    self.nodes[lrl].as_mut().unwrap().parent = l;
    self.update_left_height(id);
    self.update_right_height(l);
  }

  pub fn to_slice_preorder(&self) -> Cow<[(&K, &V)]> {
    if self.root != 0 {
      self.to_slice_preorder_from(self.root)
    } else {
      Cow::Borrowed(&[])
    }
  }

  fn to_slice_preorder_from(&self, id: usize) -> Cow<[(&K, &V)]> {
    let mut result = vec![];
    let node = self.nodes[id].as_ref().unwrap();
    result.push((&node.key, &node.value));
    if node.left != 0 {
      result.extend_from_slice(&self.to_slice_preorder_from(node.left));
    }
    if node.right != 0 {
      result.extend_from_slice(&self.to_slice_preorder_from(node.right));
    }
    result.into()
  }

  pub fn to_slice(&self) -> Cow<[(&K, &V)]> {
    if self.root != 0 {
      self.to_slice_from(self.root)
    } else {
      Cow::Borrowed(&[])
    }
  }

  fn to_slice_from(&self, id: usize) -> Cow<[(&K, &V)]> {
    let mut result = vec![];
    let node = self.nodes[id].as_ref().unwrap();
    if node.left != 0 {
      result.extend_from_slice(&self.to_slice_from(node.left));
    }
    result.push((&node.key, &node.value));
    if node.right != 0 {
      result.extend_from_slice(&self.to_slice_from(node.right));
    }
    result.into()
  }

  pub fn to_slice_postorder(&self) -> Cow<[(&K, &V)]> {
    if self.root != 0 {
      self.to_slice_postorder_from(self.root)
    } else {
      Cow::Borrowed(&[])
    }
  }

  fn to_slice_postorder_from(&self, id: usize) -> Cow<[(&K, &V)]> {
    let mut result = vec![];
    let node = self.nodes[id].as_ref().unwrap();
    if node.left != 0 {
      result.extend_from_slice(&self.to_slice_postorder_from(node.left));
    }
    if node.right != 0 {
      result.extend_from_slice(&self.to_slice_postorder_from(node.right));
    }
    result.push((&node.key, &node.value));
    result.into()
  }

  pub fn clear(&mut self) {
    self.nodes.clear();
    self.nodes.push(None);
    self.hole_ids.clear();
    self.root = 0;
  }
}

impl<K: Default, V: Default> TreeMap<K, V> {
  fn insert_root(&mut self, key: K, value: V) {
    if let Some(hole_id) = self.hole_ids.pop() {
      self.fill_hole_with_root(hole_id, key, value);
    } else {
      self.push_root(key, value);
    }
  }

  fn fill_hole_with_root(&mut self, hole_id: usize, key: K, value: V) {
    self.nodes[hole_id] = Some(Node {
      key,
      value,
      ..Default::default()
    });
    self.root = hole_id;
  }

  fn push_root(&mut self, key: K, value: V) {
    self.nodes.push(Some(Node {
      key,
      value,
      ..Default::default()
    }));
    self.root = 1;
  }
}

impl<K: PartialOrd, V> TreeMap<K, V> {
  pub fn has(&self, key: K) -> bool {
    self.get(key).is_some()
  }

  pub fn get(&self, key: K) -> Option<&V> {
    if let Some(id) = self.get_id(key) {
      Some(&self.nodes[id].as_ref().unwrap().value)
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
    if let Some(id) = self.get_id(key) {
      Some(&mut self.nodes[id].as_mut().unwrap().value)
    } else {
      None
    }
  }

  pub fn remove(&mut self, key: K) -> Option<V> {
    if let Some(old) = self.get_id(key) {
      let old_node_slot = self.nodes.get_mut(old).unwrap();
      let old_node = old_node_slot.as_ref().unwrap();
      let old_p = old_node.parent;
      let old_l = old_node.left;
      let old_r = old_node.right;
      let result = old_node_slot.take().unwrap().value;
      self.hole_ids.push(old);
      if old_l != 0 {
        let new = self.max_id_from(old_l);
        let new_node = self.nodes[new].as_ref().unwrap();
        let new_p = new_node.parent;
        let new_l = new_node.left;
        if old_p != 0 {
          let old_p_node = self.nodes[old_p].as_mut().unwrap();
          if old_p_node.left == old {
            old_p_node.left = new;
          } else {
            old_p_node.right = new;
          }
          let new_node = self.nodes[new].as_mut().unwrap();
          new_node.parent = old_p;
          if new_p != old {
            new_node.left = old_l;
          }
          new_node.right = old_r;
        } else {
          self.root = new;
          let new_node = self.nodes[new].as_mut().unwrap();
          new_node.parent = 0;
          if new_p != old {
            new_node.left = old_l;
          }
          new_node.right = old_r;
        }
        if old_r != 0 {
          self.nodes[old_r].as_mut().unwrap().parent = new;
        }
        if new_p != old {
          self.nodes[old_l].as_mut().unwrap().parent = new;
          self.nodes[new_p].as_mut().unwrap().right = new_l;
          if new_l != 0 {
            self.nodes[new_l].as_mut().unwrap().parent = new_p;
          }
          self.update(new_p);
        } else {
          self.update(new);
        }
      } else if old_r != 0 {
        let new = self.min_id_from(old_r);
        let new_node = self.nodes[new].as_ref().unwrap();
        let new_p = new_node.parent;
        let new_r = new_node.right;
        if old_p != 0 {
          let old_p_node = self.nodes[old_p].as_mut().unwrap();
          if old_p_node.left == old {
            old_p_node.left = new;
          } else {
            old_p_node.right = new;
          }
          let new_node = self.nodes[new].as_mut().unwrap();
          new_node.parent = old_p;
          new_node.left = 0;
          if new_p != old {
            new_node.right = old_r;
          }
        } else {
          self.root = new;
          let new_node = self.nodes[new].as_mut().unwrap();
          new_node.parent = 0;
          new_node.left = 0;
          if new_p != old {
            new_node.right = old_r;
          }
        }
        if new_p != old {
          self.nodes[old_r].as_mut().unwrap().parent = new;
          self.nodes[new_p].as_mut().unwrap().left = new_r;
          if new_r != 0 {
            self.nodes[new_r].as_mut().unwrap().parent = new_p;
          }
          self.update(new_p);
        } else {
          self.update(new);
        }
      } else if old_p != 0 {
        let old_p_node = self.nodes[old_p].as_mut().unwrap();
        if old_p_node.left == old {
          old_p_node.left = 0;
        } else {
          old_p_node.right = 0;
        }
        self.update(old_p);
      } else {
        self.root = 0;
      }
      Some(result)
    } else {
      None
    }
  }

  fn get_id(&self, key: K) -> Option<usize> {
    let mut cursor = self.root;
    while cursor != 0 {
      let cursor_node = self.nodes[cursor].as_ref().unwrap();
      if key < cursor_node.key {
        cursor = cursor_node.left;
      } else if key > cursor_node.key {
        cursor = cursor_node.right;
      } else {
        return Some(cursor);
      }
    }
    None
  }

  pub fn min(&self) -> Option<(&K, &V)> {
    if self.root != 0 {
      let min_node = self.nodes[self.min_id_from(self.root)].as_ref().unwrap();
      Some((&min_node.key, &min_node.value))
    } else {
      None
    }
  }

  fn min_id_from(&self, id: usize) -> usize {
    let mut cursor = id;
    loop {
      let cursor_node = self.nodes[cursor].as_ref().unwrap();
      if cursor_node.left != 0 {
        cursor = cursor_node.left;
      } else {
        return cursor;
      }
    }
  }

  pub fn max(&self) -> Option<(&K, &V)> {
    if self.root != 0 {
      let max_node = self.nodes[self.max_id_from(self.root)].as_ref().unwrap();
      Some((&max_node.key, &max_node.value))
    } else {
      None
    }
  }

  fn max_id_from(&self, id: usize) -> usize {
    let mut cursor = id;
    loop {
      let cursor_node = self.nodes[cursor].as_ref().unwrap();
      if cursor_node.right != 0 {
        cursor = cursor_node.right;
      } else {
        return cursor;
      }
    }
  }
}

impl<K: PartialOrd, V> Index<K> for TreeMap<K, V> {
  type Output = V;

  fn index(&self, key: K) -> &Self::Output {
    self
      .get(key)
      .expect("The value that is associated with the given key does not exist in the tree map!")
  }
}

impl<K: PartialOrd, V> IndexMut<K> for TreeMap<K, V> {
  fn index_mut(&mut self, key: K) -> &mut Self::Output {
    self
      .get_mut(key)
      .expect("The value that is associated with the given key does not exist in the tree map!")
  }
}

impl<K: Default + PartialOrd, V: Default> TreeMap<K, V> {
  pub fn bulk_put(&mut self, iter: impl Iterator<Item = (K, V)>) {
    for (key, value) in iter {
      self.put(key, value);
    }
  }

  pub fn put(&mut self, key: K, value: V) {
    if self.root == 0 {
      self.insert_root(key, value);
      return;
    }
    let mut cursor = self.root;
    loop {
      let cursor_node = self.nodes[cursor].as_ref().unwrap();
      if key < cursor_node.key {
        if cursor_node.left == 0 {
          let parent = cursor_node.parent;
          self.insert_left(cursor, key, value);
          self.update(parent);
          break;
        } else {
          cursor = cursor_node.left;
        }
      } else if key > cursor_node.key {
        if cursor_node.right == 0 {
          let parent = cursor_node.parent;
          self.insert_right(cursor, key, value);
          self.update(parent);
          break;
        } else {
          cursor = cursor_node.right;
        }
      } else {
        self.nodes[cursor].as_mut().unwrap().value = value;
        break;
      }
    }
  }

  fn insert_left(&mut self, id: usize, key: K, value: V) {
    if let Some(hole_id) = self.hole_ids.pop() {
      self.fill_hole_with_node_when_insert_left(id, hole_id, key, value);
    } else {
      self.push_node_when_insert_left(id, key, value);
    }
  }

  fn fill_hole_with_node_when_insert_left(&mut self, id: usize, hole_id: usize, key: K, value: V) {
    let node = self.nodes[id].as_mut().unwrap();
    node.left = hole_id;
    node.left_height += 1;
    self.nodes[hole_id] = Some(Node {
      key,
      value,
      parent: id,
      ..Default::default()
    });
  }

  fn push_node_when_insert_left(&mut self, id: usize, key: K, value: V) {
    let node_count = self.nodes.len();
    let node = self.nodes[id].as_mut().unwrap();
    node.left = node_count;
    node.left_height += 1;
    self.nodes.push(Some(Node {
      key,
      value,
      parent: id,
      ..Default::default()
    }));
  }

  fn insert_right(&mut self, id: usize, key: K, value: V) {
    if let Some(hole_id) = self.hole_ids.pop() {
      self.fill_hole_with_node_when_insert_right(id, hole_id, key, value);
    } else {
      self.push_node_when_insert_right(id, key, value);
    }
  }

  fn fill_hole_with_node_when_insert_right(&mut self, id: usize, hole_id: usize, key: K, value: V) {
    let node = self.nodes[id].as_mut().unwrap();
    node.right = hole_id;
    node.right_height += 1;
    self.nodes[hole_id] = Some(Node {
      key,
      value,
      parent: id,
      ..Default::default()
    });
  }

  fn push_node_when_insert_right(&mut self, id: usize, key: K, value: V) {
    let node_count = self.nodes.len();
    let node = self.nodes[id].as_mut().unwrap();
    node.right = node_count;
    node.right_height += 1;
    self.nodes.push(Some(Node {
      key,
      value,
      parent: id,
      ..Default::default()
    }));
  }
}

#[derive(Debug, Default)]
struct Node<K, V> {
  key: K,
  value: V,
  parent: usize,
  left: usize,
  right: usize,
  left_height: usize,
  right_height: usize,
}
