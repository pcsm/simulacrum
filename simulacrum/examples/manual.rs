extern crate simulacrum;

use simulacrum::*;

trait CoolTrait {
    // Shared self
    fn foo(&self);

    // Mutable self
    fn bar(&mut self);

    // One parameter and returning a value
    fn goop(&mut self, flag: bool) -> u32;

    // Multiple parameters
    fn zing(&self, first: i32, second: bool);

    // Static reference
    fn boop(&self, name: &'static str);
}

pub struct CoolTraitMock {
    e: Expectations
}

impl CoolTraitMock {
    pub fn new() -> Self {
        Self {
            e: Expectations::new()
        }
    }

    pub fn then(&mut self) -> &mut Self {
        self.e.then();
        self
    }

    pub fn expect_foo(&mut self) -> Method<(), ()> {
        self.e.expect::<(), ()>("foo")
    }

    pub fn expect_bar(&mut self) -> Method<(), ()> {
        self.e.expect::<(), ()>("bar")
    }

    pub fn expect_goop(&mut self) -> Method<bool, u32> {
        self.e.expect::<bool, u32>("goop")
    }

    pub fn expect_zing(&mut self) -> Method<(i32, bool), ()> {
        self.e.expect::<(i32, bool), ()>("zing")
    }

    pub fn expect_boop(&mut self) -> Method<&'static str, ()> {
        self.e.expect::<&'static str, ()>("boop")
    }
}

impl CoolTrait for CoolTraitMock {
    fn foo(&self) {
        self.e.was_called::<(), ()>("foo", ())
    }

    fn bar(&mut self) {
        self.e.was_called::<(), ()>("bar", ())
    }

    fn goop(&mut self, flag: bool) -> u32 {
        self.e.was_called_returning::<bool, u32>("goop", flag)
    }

    fn zing(&self, first: i32, second: bool) {
        self.e.was_called::<(i32, bool), ()>("zing", (first, second))
    }

    fn boop(&self, name: &'static str) {
        self.e.was_called::<&'static str, ()>("boop", name)
    }
}

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_bar().called_never();
    m.expect_foo().called_once();
    m.then().expect_goop().called_once().with(|&arg| arg == true).returning(|_| 5);
    m.then().expect_zing().called_once().with(|args| args.0 == 13 && args.1 == false);
    m.expect_boop().called_times(2);

    // Execute test code
    m.foo();
    assert_eq!(m.goop(true), 5);
    m.zing(13, false);
    m.boop("hey");
    m.boop("yo");

    // When the Expectations struct is dropped, each of its expectations will be evaluated
}