//! Functionality that is helpful when using mock objects created with Simulacrum.

extern crate debugit;
extern crate simulacrum_shared;

#[macro_use]
pub mod macros;
pub mod validators;

pub use self::validators::*;