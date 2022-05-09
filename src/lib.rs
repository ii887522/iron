pub mod any;
pub mod functions;
pub mod reactive;
pub mod structs;

pub use any::*;
pub use functions::*;
pub use reactive::*;
pub use structs::*;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
