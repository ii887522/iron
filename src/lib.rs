//! It is a general-purpose library for [Rust](https://www.rust-lang.org/) that can help developers create various kinds
//! of applications in a shorter amount of time. This library is also used to serve as a custom extension to the
//! [Rust](https://www.rust-lang.org/) standard library.

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
