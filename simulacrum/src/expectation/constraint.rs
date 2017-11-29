use super::result::{ExpectationError, ExpectationResult};

/// The `Constraint`s attatched to an `Expectation` must all pass in order for the
/// `Excpectation` to also pass.
pub enum Constraint<I> {
    /// A method must be called with parameters that meet certain requirements.
    /// The data member is a closure that can be called with the params to verify this.
    Params(Box<FnMut(I) -> bool>),
    /// A method must be called a certain number of times
    Times(i64),
    /// For testing
    AlwaysPass,
    /// For testing
    AlwaysFail
}

impl<I> Constraint<I> where
    I: 'static 
{
    fn verify(&self) -> ExpectationResult {
        match self {
            &Constraint::AlwaysFail => Err(ExpectationError::AlwaysFail),
            _ => Ok(())
        }
    }
}

/*
impl Expectation {
    pub fn validatemmm(&mut self) -> ExpectationResult {
        match self {
            &mut Expectation::CallArgs(key, boxed_t) => {
                boxed_t.validate()
            },
            &mut Expectation::CallTimes(key, times) => {
                match times {
                    x if x < 0 => Err(ExpectationError::CalledTooManyTimes(key, x.abs())),
                    x if x > 0 => Err(ExpectationError::CalledTooFewTimes(key, x)),
                    _ => Ok(())
                }
            },
        }
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_pass() {
        let c: Constraint<()> = Constraint::AlwaysPass;

        assert!(c.verify().is_ok(), "Constraint should always pass");
    }

    #[test]
    fn test_always_fail() {
        let c: Constraint<()> = Constraint::AlwaysFail;

        assert!(c.verify().is_err(), "Constraint should always fail");
    }

    #[test]
    fn test_times_pass() {

    }

    #[test]
    fn test_times_fail_called_fewer() {

    }

    #[test]
    fn test_times_fail_called_more() {

    }

    #[test]
    fn test_params_pass() {

    }

    #[test]
    fn test_params_fail() {

    }
}