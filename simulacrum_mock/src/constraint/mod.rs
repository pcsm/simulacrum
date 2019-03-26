pub mod result;
pub mod stock;

pub use self::result::*;

/// A `Constraint` is a type that can be added to an `Expectation`.
///
/// All `Constraint`s added to an `Expectation` must all pass in order for the
/// `Expectation` to pass.
pub trait Constraint<I> {
    /// This constraint has been called with the given parameters. Update the
    ///
    /// Constraint state so that when `verify()` is called, it will return the
    /// correct result.
    #[allow(unused_variables)]
    fn handle_call(&mut self, params: &I) {}

    /// At the end of the test, see if the Constraint passed or failed.
    fn verify(&self) -> ConstraintResult;
}

// lol, it would be handy to have simulacrum here
pub struct ConstraintMock {
    handle_call_expected: bool,
    handle_call_called: bool,
}

impl ConstraintMock {
    pub fn new() -> Self {
        Self {
            handle_call_expected: false,
            handle_call_called: false,
        }
    }

    pub fn expect_handle_call(&mut self) {
        self.handle_call_expected = true
    }
}

impl<I> Constraint<I> for ConstraintMock {
    fn handle_call(&mut self, _params: &I) {
        self.handle_call_called = true
    }

    fn verify(&self) -> ConstraintResult {
        Ok(())
    }
}

impl Drop for ConstraintMock {
    /// All expectations will be verified when the mock object is dropped.
    fn drop(&mut self) {
        if self.handle_call_expected && !self.handle_call_called {
            panic!("handle_call was not called");
        }
    }
}
