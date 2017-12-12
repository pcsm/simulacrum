use debugit::DebugIt;

use constraint::{Constraint, ConstraintError, ConstraintResult};
use validator::Validator;

/// A method must be called with parameters that meet certain requirements.
pub struct Params<I> {
    /// Should be `true` if the method has been called with valid parameters every time.
    is_valid: bool,
    invalid_param_msg: String,
    /// A closure that will be called with the parameters to validate that they 
    /// conform to the requirements.
    validator: Box<Validator<I>>
}

impl<I> Params<I> {
    pub fn new<V>(validator: V) -> Self where
        V: Validator<I> + 'static
    {
        Params {
            is_valid: true,
            invalid_param_msg: "".to_owned(),
            validator: Box::new(validator)
        }
    }
}

impl<I> Constraint<I> for Params<I> {
    fn handle_call(&mut self, params: &I) {
        if self.is_valid {
            self.is_valid = self.validator.validate(params);
            if !self.is_valid {
                self.invalid_param_msg = format!("{:?}", DebugIt(params));
            }
        }
    }

    fn verify(&self) -> ConstraintResult {
        if self.is_valid {
            Ok(())
        } else {
            Err(ConstraintError::MismatchedParams(self.invalid_param_msg.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::stock::trivial::*;
    use validator::stock::check::*;

    #[test]
    fn test_new() {
        let c = Params::new(none());

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should pass after being created");
    }

    #[test]
    fn test_handle_call_pass() {
        // Validator approves of any input
        let mut c = Params::new(any());

        c.handle_call(&());
        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should pass");
    }

    #[test]
    // Note: This test will fail on stable rust b/c the debugit crate relies on
    // core_intrinsics & specialization features.
    fn test_handle_call_fail() {
        // Validator closure disapproves of any input
        let mut c = Params::new(none());

        c.handle_call(&());
        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::MismatchedParams("()".to_owned()), "Constraint should return the correct error");
    }

    #[test]
    fn test_handle_call_good_then_bad() {
        // Validator closure approves input over 5
        let mut c = Params::new(passes(|arg| *arg > 5));

        c.handle_call(&10); // Good
        c.handle_call(&3); // Bad
        let r = <Constraint<i32>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
    }

    #[test]
    fn test_handle_call_bad_then_good() {
        // Validator closure approves input over 5
        let mut c = Params::new(passes(|arg| *arg > 5));

        c.handle_call(&3); // Bad
        c.handle_call(&10); // Good
        let r = <Constraint<i32>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
    }
}