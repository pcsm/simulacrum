use constraint::{Constraint, ConstraintError, ConstraintResult};

/// A method must be called with parameters that meet certain requirements.
pub struct Params<I> {
    /// Should be `true` if the method has been called with valid parameters every time.
    is_valid: bool,
    /// A closure that will be called with the parameters to validate that they 
    /// conform to the requirements.
    validator: Box<FnMut(&I) -> bool>
}

impl<I> Params<I> {
    pub fn new<F>(validator: F) -> Self where
        F: FnMut(&I) -> bool + 'static
    {
        Params {
            is_valid: true,
            validator: Box::new(validator)
        }
    }
}

impl<I> Constraint<I> for Params<I> {
    fn handle_call(&mut self, params: &I) {
        if self.is_valid {
            self.is_valid = (self.validator)(params);
        }
    }

    fn verify(&self) -> ConstraintResult {
        if self.is_valid {
            Ok(())
        } else {
            Err(ConstraintError::MismatchedParams)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = Params::new(|_| false);

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should pass after being created");
    }

    #[test]
    fn test_handle_call_pass() {
        // Validator closure approves of any input
        let mut c = Params::new(|_| true);

        c.handle_call(&());
        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should pass");
    }

    #[test]
    fn test_handle_call_fail() {
        // Validator closure disapproves of any input
        let mut c = Params::new(|_| false);

        c.handle_call(&());
        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::MismatchedParams, "Constraint should return the correct error");
    }

    #[test]
    fn test_handle_call_good_then_bad() {
        // Validator closure approves input over 5
        let mut c = Params::new(|arg| *arg > 5);

        c.handle_call(&10); // Good
        c.handle_call(&3); // Bad
        let r = <Constraint<i32>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
    }

    #[test]
    fn test_handle_call_bad_then_good() {
        // Validator closure approves input over 5
        let mut c = Params::new(|arg| *arg > 5);

        c.handle_call(&3); // Bad
        c.handle_call(&10); // Good
        let r = <Constraint<i32>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
    }
}