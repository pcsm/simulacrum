pub mod result;
pub mod stock;

pub use self::result::*;

/// A `Constraint` is a type that can be added to an `Expectation`.
///
/// All `Constraint`s added th to an `Expectation` must all pass in order for the
/// `Expectation` to pass.
pub trait Constraint<I> {
    /// This constraint has been called with the given parameters. Update the
    ///
    /// Constraint state so that when `verify()` is called, it will return the
    /// correct result.
    fn handle_call(&mut self, params: I) { }

    /// At the end of the test, see if the Constraint passed or failed.
    fn verify(&self) -> ConstraintResult;
}
