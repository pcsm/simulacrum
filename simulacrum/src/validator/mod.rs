use debugit::DebugIt;

pub mod stock;

/// A `Validator` is an object that knows how to validate method parameters.
///
/// To use these, you typically pass them to the `.with()` method for use with
/// the `Params` Constraint.
pub trait Validator<I> {
    /// This object has been called with the given parameters. Return `true`
    /// if they are acceptable, and `false` if they are not.
    fn validate(&mut self, param: &I) -> bool;

    // Method to create a string representing the pass condition of this Validator.
    // We do this so that types that implement `PartialEq` (and are therefore 
    // `Validator`s), but do not implement Debug can still be printed using `DebugIt`.
    fn print(&self) -> String;
}


/// `Validator` is automatically implemented for types that implement `PartialEq`.
impl<I: PartialEq> Validator<I> for I {
    fn validate(&mut self, param: &I) -> bool {
        &*param == self
    }

     fn print(&self) -> String {
         format!("{:?}", DebugIt(self)).to_owned()
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let v = 555;
        assert!(555.validate(&v));
    }
}