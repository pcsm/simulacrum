extern crate debugit;
extern crate handlebox;

#[macro_use]
pub mod macros;

pub mod constraint;
pub mod expectation;
pub mod user;
pub mod mock;
pub mod validator;
mod store;

pub type MethodName = &'static str;

pub use handlebox::Handle as ExpectationId;

pub use self::mock::Expectations;
pub use self::user::Method;
pub use self::validator::stock::*;