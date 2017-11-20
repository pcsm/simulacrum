extern crate simulacrum;

use simulacrum::*;

trait CoolTrait {
    fn foo(&self);
    fn bar(&mut self);
    fn goop(&mut self, flag: bool) -> u32;
}

pub struct CoolTraitMock {
    expectations: ExpectationStore
}

impl CoolTraitMock {
    pub fn new() -> Self {
        Self {
            expectations: ExpectationStore::new()
        }
    }

    pub fn expect(&mut self, name: &'static str) -> Box<TrackedMethodT> {
        self.expectations.track_method(name)
    }

    pub fn expect_foo(&mut self) -> TrackedMethod<(), ()> {
        self.expect("foo").downcast::<TrackedMethod<(), ()>>().unwrap()
    }

    pub fn expect_bar(&mut self) -> TrackedMethod<(), ()> {
        self.expect("bar").downcast::<TrackedMethod<(), ()>>().unwrap()
    }

    pub fn expect_goop(&mut self) -> TrackedMethod<(bool), u32> {
        self.expect("goop").downcast::<TrackedMethod<(bool), u32>>().unwrap()
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        self.expectations
            .was_called("foo")
            .downcast::<TrackedMethodData<(), ()>>()
            .unwrap()
            .with(())
            .returning()
    }

    fn bar(&mut self) {
        self.expectations
            .was_called("bar")
            .downcast::<TrackedMethodData<(), ()>>()
            .unwrap()
            .with(())
            .returning()
    }

    fn goop(&mut self, flag: bool) -> u32 {
        self.expectations
            .was_called("goop")
            .downcast::<TrackedMethodData<(bool), u32>>()
            .unwrap()
            .with((flag))
            .returning()
    }
}

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_foo().called_once();
    m.expect_bar().called_never();
    m.expect_goop().called_once().with(true).returning(|| 5);

    // Execute test code
    m.foo();
    assert_eq!(m.goop(true), 5);

    // When the ExpectationStore is dropped, the expectations will be evaluated
}