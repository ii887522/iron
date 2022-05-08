use std::borrow::Cow;

pub trait Hash
where
  [Self::Part]: ToOwned,
{
  type Part;

  fn hash(&self) -> Cow<[Self::Part]>;
}
