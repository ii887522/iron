pub mod any;
pub mod functions;
pub mod reactive;
pub mod r#struct;

pub use any::*;
pub use functions::*;
pub use r#struct::*;
pub use reactive::*;

use std::cell::RefCell;
use std::rc::Rc;

pub type Shared<T> = Rc<RefCell<T>>;
