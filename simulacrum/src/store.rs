use std::collections::HashMap;
use std::sync::Mutex;

use super::{Expectation, ExpectationId}

pub(crate) struct ExpectationsStore(Mutex<HashMap<ExpectationId, Box<Expectation>>>);

impl ExpectationsStore {
    fn new() -> Self {
        ExpectationsStore(Mutex::new(HashMap::new()))
    }
}
