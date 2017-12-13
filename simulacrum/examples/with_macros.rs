#[macro_use]
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

    // Shared reference
    fn store(&self, val: &i64);

    // Mutable reference
    fn toggle(&self, bit: &mut bool);
}

create_mock! {
    struct CoolTraitMock: {
        expect_foo("foo");
        expect_bar("bar");
        expect_goop("goop") bool => u32;
        expect_zing("zing") (i32, bool);
        expect_boop("boop") &'static str;
        expect_store("store") *const i64;
        expect_toggle("toggle") *mut bool;
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

    fn store(&self, val: &i64) {
        self.e.was_called::<*const i64, ()>("store", val)
    }

    fn toggle(&self, bit: &mut bool) {
        self.e.was_called_returning::<*mut bool, ()>("toggle", bit)
    }
}

fn main() {
    // Set up expectations
    let mut m = CoolTraitMock::new();
    m.expect_bar().called_never();
    m.expect_foo().called_once();
    m.then().expect_goop().called_once().with(true).returning(|_| 5);
    m.then().expect_zing().called_once().with(params!(13, false));
    m.expect_boop().called_times(2);
    m.expect_store().called_once().with(deref(777));
    m.expect_toggle().called_once().with(deref(true))
                                   .returning(|&arg| { unsafe { *arg.as_mut().unwrap() = false } });

    // Execute test code
    m.foo();
    assert_eq!(m.goop(true), 5);
    m.zing(13, false);
    m.boop("hey");
    m.boop("yo");
    m.store(&777);
    let mut b = true;
    m.toggle(&mut b);
    assert_eq!(b, false);

    // When the Expectations struct is dropped, each of its expectations will be evaluated
}