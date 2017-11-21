use handlebox::HandleBox;

use std::any::Any;
use std::marker::PhantomData;
use std::sync::Mutex;

use super::ExpectationId;
use super::expectation::{CallExpectation, Expectation, ExpectationResult};

// A HandleBox of Expectations, with one of them being a top-level `Group` that every
// other Expectation is a member of.
pub(crate) struct ExpectationsStore {
    mutex: Mutex<HandleBox<Expectation>>,
    top_group: ExpectationId
}

impl ExpectationsStore {
    pub fn new() -> Self {
        let mut hb = HandleBox::new();
        let top_group = hb.add(Expectation::new_group());
        ExpectationsStore {
            mutex: Mutex::new(hb),
            top_group
        }
    }

    pub fn get_mut(&self, id: ExpectationId) -> ExpectationEditor {
        ExpectationEditor {
            id,
            store: &self
        }
    }

    // Add a new Expectation under the top-level `Group` and return its id.
    pub fn add(&mut self, expectation: Expectation) -> ExpectationId {
        let id = self.mutex.lock().unwrap().add(expectation);
        self.get_mut(self.top_group).add_to_group(id);
        id
    }

    // Validate all expectations in this store.
    pub fn validate(&self) -> ExpectationResult {
        self.get_mut(self.top_group).validate()
    }
}

// Used internally to mutably access an `ExpectationStore`.
pub struct ExpectationEditor<'a> {
    id: ExpectationId,
    store: &'a ExpectationsStore
}

impl<'a> ExpectationEditor<'a> {
    fn add_to_group(&self, id: ExpectationId) {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().add_to_group(id);
    }

    pub(crate) fn add_to_call(&self, c_exp: CallExpectation) {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().add_to_call(c_exp);
    }

    pub(crate) fn set_call_return(&mut self, return_behavior: Box<Any>) {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().set_call_return(return_behavior);
    }

    fn validate(&self) -> ExpectationResult {
        self.store.mutex.lock().unwrap().get_mut(&self.id).unwrap().validate()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//     }
// }