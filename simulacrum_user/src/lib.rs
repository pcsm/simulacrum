//! Functionality that is helpful when using mock objects created with Simulacrum.

extern crate debugit;
extern crate simulacrum_shared;

#[macro_use]
pub mod macros;

/// Validators for checking method parameters
pub mod validators;

/// `use simulacrum_user::prelude::*` to import everything in the crate except for macros
pub mod prelude {
    pub use validators::*;
}