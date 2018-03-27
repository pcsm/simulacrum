//! Types that impl `Validator`.

pub mod check;
pub mod compare;
pub mod deref;
pub mod trivial;
pub mod tuple;

pub use self::check::passes;
pub use self::compare::{gt, lt};
pub use self::deref::deref;
pub use self::trivial::{any, none};
pub use self::tuple::*;