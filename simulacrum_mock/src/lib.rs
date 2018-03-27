//! Core functionality for creating mock objects with Simulacrum.

extern crate debugit;
extern crate handlebox;
extern crate simulacrum_shared;

#[cfg(test)]
extern crate simulacrum_user;

pub mod constraint;
pub mod expectation;
pub mod method;
pub mod mock;
mod store;

pub type MethodName = &'static str;

pub use handlebox::Handle as ExpectationId;

pub use self::mock::Expectations;
pub use self::method::Method;