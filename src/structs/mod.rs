//! It contains a bunch of simple plain old structures with some helper functions applied on them to provide some
//! features.
//!
//! Modules below are useful for the user to group related variables inside structures or functions to
//! reduce the number of states for the user to keep track of during development and reduce the number of parameters
//! required to be passed into the functions to make the API more concise and clear.

pub mod bound;
pub mod seq;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub use bound::*;
pub use seq::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
