use std::collections::HashMap;
use std::sync::Mutex;

use super::ExpectationId;
use super::expectation::Expectation;

pub(crate) struct ExpectationsStore(Mutex<HashMap<ExpectationId, Box<Expectation>>>);

impl ExpectationsStore {
    pub fn new() -> Self {
        ExpectationsStore(Mutex::new(HashMap::new()))
    }
}
