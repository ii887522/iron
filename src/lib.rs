pub mod collections;
pub mod delayed;
pub mod dynamic;
pub mod fs;
pub mod hash;
pub mod id_manager;
pub mod late;
pub mod reactive;
pub mod reactivity_manager;
pub mod readonly;

pub use delayed::Delayed;
pub use dynamic::Dynamic;
pub use hash::Hash;
pub use id_manager::IDManager;
pub use late::Late;
pub use reactive::Reactive;
pub use reactivity_manager::ReactivityManager;
pub use readonly::Readonly;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
