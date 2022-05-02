extern crate iron;

use iron::IDManager;

#[test]
fn test_next() {
  let mut id_manager = IDManager::new();
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(1));
  assert_eq!(id_manager.next(), Some(2));
  id_manager.free(0).unwrap();
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(4));
  assert_eq!(id_manager.next(), Some(5));
  id_manager.free(0).unwrap();
  id_manager.free(3).unwrap();
  id_manager.free(5).unwrap();
  assert_eq!(id_manager.next(), Some(5));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(6));
  assert_eq!(id_manager.next(), Some(7));
  id_manager.free(0).unwrap();
  id_manager.free(7).unwrap();
  assert_eq!(id_manager.next(), Some(7));
  id_manager.free(7).unwrap();
  id_manager.free(3).unwrap();
  id_manager.free(4).unwrap();
  assert_eq!(id_manager.next(), Some(4));
  id_manager.free(4).unwrap();
  id_manager.free(1).unwrap();
  id_manager.free(6).unwrap();
  assert_eq!(id_manager.next(), Some(6));
  assert_eq!(id_manager.next(), Some(1));
  assert_eq!(id_manager.next(), Some(4));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(7));
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(8));
  assert_eq!(id_manager.next(), Some(9));
}

#[test]
fn test_free() {
  let mut id_manager = IDManager::new();
  id_manager.next();
  assert!(id_manager.free(10).is_err());
  assert!(id_manager.free(0).is_ok());
}
