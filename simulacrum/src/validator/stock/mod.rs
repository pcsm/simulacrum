//! Types that impl `Validator` that are included with Simulacrum.

pub mod check;
pub mod compare;
pub mod deref;
pub mod trivial;
pub mod tuple;

pub use check::passes;
pub use compare::{gt, lt};
pub use deref::deref;
pub use trivial::{any, none};