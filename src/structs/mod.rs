//! It contains a bunch of simple plain old structures with some helper functions applied on them to provide some
//! features.
//!
//! Modules below are useful for the user to group related variables inside structures or functions to
//! reduce the number of states for the user to keep track of during development and reduce the number of parameters
//! required to be passed into the functions to make the API more concise and clear.

pub mod f64_bound;
pub mod f64_seq;
pub mod f64_vec2;
pub mod f64_vec3;
pub mod f64_vec4;
pub mod i32_bound;
pub mod i32_seq;
pub mod i32_vec2;
pub mod i32_vec3;
pub mod i32_vec4;

pub use f64_bound::F64Bound;
pub use f64_seq::F64Seq;
pub use f64_vec2::F64Vec2;
pub use f64_vec3::F64Vec3;
pub use f64_vec4::F64Vec4;
pub use i32_bound::I32Bound;
pub use i32_seq::I32Seq;
pub use i32_vec2::I32Vec2;
pub use i32_vec3::I32Vec3;
pub use i32_vec4::I32Vec4;
