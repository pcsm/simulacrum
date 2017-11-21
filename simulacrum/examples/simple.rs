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

    pub fn expect(&mut self, name: &'static str) -> Box<MethodT> {
        self.expectations.track_method(name)
    }

    pub fn expect_foo(&mut self) -> Method<(), ()> {
        self.expect("foo").downcast::<Method<(), ()>>().unwrap()
    }

    pub fn expect_bar(&mut self) -> Method<(), ()> {
        self.expect("bar").downcast::<Method<(), ()>>().unwrap()
    }

    pub fn expect_goop(&mut self) -> Method<(bool), u32> {
        self.expect("goop").downcast::<Method<(bool), u32>>().unwrap()
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        self.expectations
            .was_called("foo")
            .downcast::<MethodData<(), ()>>()
            .unwrap()
            .with(())
            .returning()
    }

    fn bar(&mut self) {
        self.expectations
            .was_called("bar")
            .downcast::<MethodData<(), ()>>()
            .unwrap()
            .with(())
            .returning()
    }

    fn goop(&mut self, flag: bool) -> u32 {
        self.expectations
            .was_called("goop")
            .downcast::<MethodData<(bool), u32>>()
            .unwrap()
            .with((flag))
            .returning()
    }
}

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_bar().called_never();
    m.expect_foo().called_once();
    m.then().expect_goop().called_once().with(|args| args == true).returning(|_| 5);

    // Execute test code
    m.foo();
    assert_eq!(m.goop(true), 5);

    // When the ExpectationStore is dropped, the expectations will be evaluated
}