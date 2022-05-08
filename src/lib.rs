pub mod collections;
pub mod delayed;
pub mod dynamic;
pub mod fs;
pub mod hash;
pub mod id_manager;
pub mod late;
pub mod reactive;
pub mod reactive_slice;
pub mod reactivity_manager;
pub mod readonly;
pub mod str;

pub use delayed::Delayed;
pub use dynamic::Dynamic;
pub use hash::Hash;
pub use id_manager::IDManager;
pub use late::Late;
pub use reactive::Reactive;
pub use reactive_slice::ReactiveSlice;
pub use reactivity_manager::ReactivityManager;
pub use readonly::Readonly;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
