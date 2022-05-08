extern crate iron;

use iron::IDManager;

#[test]
fn test_id_manager() {
  let mut id_manager = IDManager::new();
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(1));
  assert_eq!(id_manager.next(), Some(2));
  assert!(id_manager.free(0).is_ok());
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(4));
  assert_eq!(id_manager.next(), Some(5));
  assert!(id_manager.free(0).is_ok());
  assert!(id_manager.free(3).is_ok());
  assert!(id_manager.free(5).is_ok());
  assert_eq!(id_manager.next(), Some(5));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(6));
  assert_eq!(id_manager.next(), Some(7));
  assert!(id_manager.free(0).is_ok());
  assert!(id_manager.free(7).is_ok());
  assert_eq!(id_manager.next(), Some(7));
  assert!(id_manager.free(7).is_ok());
  assert!(id_manager.free(3).is_ok());
  assert!(id_manager.free(4).is_ok());
  assert_eq!(id_manager.next(), Some(4));
  assert!(id_manager.free(4).is_ok());
  assert!(id_manager.free(1).is_ok());
  assert!(id_manager.free(6).is_ok());
  assert_eq!(id_manager.next(), Some(6));
  assert_eq!(id_manager.next(), Some(1));
  assert_eq!(id_manager.next(), Some(4));
  assert_eq!(id_manager.next(), Some(3));
  assert_eq!(id_manager.next(), Some(7));
  assert_eq!(id_manager.next(), Some(0));
  assert_eq!(id_manager.next(), Some(8));
  assert_eq!(id_manager.next(), Some(9));
  assert!(id_manager.free(10).is_err());
}
