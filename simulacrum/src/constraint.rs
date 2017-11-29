/// The `Constraint`s attatched to an `Expectation` must all pass in order for the
/// `Excpectation` to also pass.
pub enum Constraint<I> where
    I: 'static
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_pass() {
    }

    #[test]
    fn test_always_fail() {
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