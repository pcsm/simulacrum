//! Minimal library for creating mock objects by hand using stable Rust. 
//! 
//! This crate is a facade that just re-exports any crates necessary to both
//! create and use mock objects in Simulacrum.

extern crate simulacrum_macros;
extern crate simulacrum_mock;
extern crate simulacrum_user;

// pub use * from the crate root re-exports macros since Rust 1.15
pub use simulacrum_macros::*;
pub use simulacrum_mock::*;
pub use simulacrum_user::*;