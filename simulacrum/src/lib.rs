//! Minimal library for creating mock objects by hand using stable Rust. 
//! 
//! This crate is a facade that just re-exports any crates necessary to both
//! create and use mock objects in Simulacrum.

extern crate simulacrum_macros;
extern crate simulacrum_mock;
extern crate simulacrum_user;

// Importing * from the crate root re-exports macros since Rust 1.15
pub use simulacrum_macros::*;
pub use simulacrum_user::*;

/// `use simulacrum::prelude::*` to import everything in this crate except for macros
pub mod prelude {
    pub use ::mock::*;
    pub use ::user::*;
}

/// For creating mock objects.
pub mod mock {
    pub use simulacrum_mock::*;
}

/// For using mock objects.
pub mod user {
    pub use simulacrum_user::prelude::*;
}