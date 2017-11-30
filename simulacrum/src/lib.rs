extern crate handlebox;

pub mod constraint;
pub mod expectation;
pub mod user;
pub mod mock;
mod store;

pub type MethodName = &'static str;

pub use handlebox::Handle as ExpectationId;

pub use self::mock::Expectations;
pub use self::user::Method;
