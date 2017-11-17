extern crate simulacrum;

use simulacrum::*;

trait CoolTrait {
    // Unfortunately, right now simulacrum only helps to mock mutable methods
    fn foo(&self);
    fn bar(&mut self);
}

pub struct CoolTraitMock {
    expectations: ExpectationStore<&'static str>
}

impl CoolTraitMock {
    pub fn new() -> Self {
        Self {
            expectations: ExpectationStore::new()
        }
    }

    pub fn expect(&mut self, name: &'static str) -> TrackedMethod<&'static str> {
        self.expectations.track_method(name, name)
    }

    pub fn expect_foo(&mut self) -> TrackedMethod<&'static str> {
        self.expect("foo")
    }

    pub fn expect_bar(&mut self) -> TrackedMethod<&'static str> {
        self.expect("bar")
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        self.expectations.was_called("foo");
    }

    fn bar(&mut self) {
        self.expectations.was_called("bar");
    }
}

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_bar().called_once();
    m.expect_foo().called_never();

    // Execute test code
    m.bar();
    m.foo();

    // When the ExpectationStore is dropped, the expectations will be evaluated
}