use constraint::{Constraint, ConstraintError, ConstraintResult};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}