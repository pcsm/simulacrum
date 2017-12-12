pub mod stock;

/// A `Validator` is an object that knows how to validate method parameters.
///
/// To use these, you typically pass them to the `.with()` method for use with
/// the `Params` Constraint.
pub trait Validator<I> {
    /// This object has been called with the given parameters. Return `true`
    /// if they are acceptable, and `false` if they are not.
    fn validate(&mut self, param: &I) -> bool;
}