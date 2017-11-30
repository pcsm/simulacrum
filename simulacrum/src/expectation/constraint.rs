use std::fmt;

/// The Error type is a message to be printed to the user.
pub type ConstraintResult = Result<(), ConstraintError>;

#[derive(Debug, PartialEq)]
pub enum ConstraintError {
    AlwaysFail,
    CalledTooFewTimes(i64),
    CalledTooManyTimes(i64),
    CallNotExpected,
    Custom(String), // For custom constraints from users
    MismatchedParams,
}

use self::ConstraintError::*;

impl fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ConstraintError::AlwaysFail => {
                write!(f, "Expectation will always fail")
            },
            &ConstraintError::CalledTooFewTimes(times) => {
                write!(f, "Called {} times fewer than expected", times)
            },
            &ConstraintError::CalledTooManyTimes(times) => {
                write!(f, "Called {} times more than expected", times)
            },
            &ConstraintError::CallNotExpected => {
                write!(f, "Called when not expected")
            },
            &ConstraintError::Custom(ref msg) => {
                write!(f, "{}", msg)
            },
            &ConstraintError::MismatchedParams => {
                write!(f, "Called with unexpected parameters")
            },
        }
    }
}

/// The `Constraint`s attatched to an `Expectation` must all pass in order for the
/// `Expectation` to also pass.
pub trait Constraint<I> {
    /// This constraint has been called with the given parameters. Update the
    ///
    /// Constraint state so that when `verify()` is called, it will return the
    /// correct result.
    fn handle_call(&mut self, params: I) { }

    /// At the end of the test, see if the Constraint passed or failed.
    fn verify(&self) -> ConstraintResult;
}

/// For testing
pub struct AlwaysPass;

impl<I> Constraint<I> for AlwaysPass {
    fn verify(&self) -> ConstraintResult {
        Ok(())
    }
}

/// For testing
pub struct AlwaysFail;

impl<I> Constraint<I> for AlwaysFail {
    fn verify(&self) -> ConstraintResult {
        Err(ConstraintError::AlwaysFail)
    }
}

/// A method must be called a certain number of times
pub struct Times(i64);

impl Times {
    pub fn new(expected_calls: i64) -> Self {
        Times(expected_calls)
    }
}

impl<I> Constraint<I> for Times {
    fn handle_call(&mut self, params: I) {
        self.0 -= 1;
    }

    fn verify(&self) -> ConstraintResult {
        match self.0 {
            x if x < 0 => Err(ConstraintError::CalledTooManyTimes(x.abs())),
            x if x > 0 => Err(ConstraintError::CalledTooFewTimes(x)),
            _ => Ok(())
        }
    }
}

/// A method must be called with parameters that meet certain requirements.
pub struct Params<I> {
    /// Should be `true` if the method has been called with valid parameters every time.
    is_valid: bool,
    /// A closure that will be called with the parameters to validate that they 
    /// conform to the requirements.
    validator: Box<FnMut(I) -> bool>
}

impl<I> Params<I> {
    pub fn new<F>(validator: F) -> Self where
        F: FnMut(I) -> bool + 'static
    {
        Params {
            is_valid: true,
            validator: Box::new(validator)
        }
    }
}

impl<I> Constraint<I> for Params<I> {
    fn handle_call(&mut self, params: I) {
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
    fn test_always_pass() {
        let c: AlwaysPass = AlwaysPass;

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should always pass");
    }

    #[test]
    fn test_always_fail() {
        let c = AlwaysFail;

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should always fail");
        assert_eq!(r.unwrap_err(), ConstraintError::AlwaysFail, "Constraint should return the correct error");
    }

    #[test]
    fn test_times_pass() {
        let c = Times::new(0);

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok());
    }

    #[test]
    fn test_times_fail_called_fewer() {
        let c = Times::new(1);

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::CalledTooFewTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_times_fail_called_more() {
        let c = Times::new(-1);

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_err(), "Constraint should fail");
        assert_eq!(r.unwrap_err(), ConstraintError::CalledTooManyTimes(1), "Constraint should return the correct error");
    }

    #[test]
    fn test_params_pass() {
        let c = Params::new(|_| false);

        let r = <Constraint<()>>::verify(&c);

        assert!(r.is_ok(), "Constraint should pass");
    }

    // #[test]
    // fn test_params_fail() {
    //     let c = Params::new(|_| false);

    // let r = <Constraint<()>>::verify(&c);

    //     assert!(r.is_err(), "Constraint should fail");
    //     assert_eq!(r.unwrap_err(), ConstraintError::MismatchedParams, "Constraint should return the correct error");
    // }
}