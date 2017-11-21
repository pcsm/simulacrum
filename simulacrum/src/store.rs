use handlebox::HandleBox;

use std::marker::PhantomData;
use std::sync::Mutex;

use super::ExpectationId;
use super::expectation::{CallExpectation, Expectation, ExpectationResult};

// A HandleBox of Expectations, with one of them being a top-level `All` that every
// other Expectation is a member of.
pub(crate) struct ExpectationsStore {
    mutex: Mutex<HandleBox<Expectation>>,
    top: ExpectationId
}

impl ExpectationsStore {
    pub fn new() -> Self {
        let mut hb = HandleBox::new();
        let top = hb.add(Expectation::new_all());
        ExpectationsStore {
            mutex: Mutex::new(hb),
            top
        }
    }

    pub fn get_mut(&self, id: ExpectationId) -> ExpectationEditor {
        ExpectationEditor {
            id,
            store: &self
        }
    }

    // Add a new Expectation under the top-level `All` and return its id.
    pub fn add(&mut self, expectation: Expectation) -> ExpectationId {
        let id = self.mutex.lock().unwrap().add(expectation);
        self.get_mut(self.top).add_to_all(id);
        id
    }

    // Validate all expectations in this store.
    pub fn validate(&self) -> ExpectationResult {
        self.get_mut(self.top).validate()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a> {
    id: ExpectationId,
    store: &'a ExpectationsStore
}

impl<'a> ExpectationEditor<'a> {
    fn add_to_all(&self, id: ExpectationId) {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().add_to_all(id);
    }

    fn add_to_call(&self, c_exp: CallExpectation) {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().add_to_call(c_exp);
    }

    fn validate(&self) -> ExpectationResult {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().validate()
    }
}
