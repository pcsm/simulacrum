use super::{ExpectationId, ExpectationResult};

/// A set of expectations that should be met at the same time.
///
/// Calling `Expectations.then()` creates a new era.
/// 
/// All expectations in an era must be met before the next era is evaluated.
pub(crate) struct Era(Vec<ExpectationId>);

impl Era {
    pub(crate) fn new() -> Self {
        Era(Vec::new())
    }

    pub(crate) fn add(&mut self, id: ExpectationId) {
        self.0.push(id)
    }

    pub(crate) fn verify(&self) -> ExpectationResult {
        Ok(())
    }
}