// Here we'll use the mid-level macros available in Simulacrum.
//
// There is one to create the Mock struct itself, and one to facilitate
// marking methods as called when implementing the trait for the Mock struct.
//
// Note that if you want more control over your Mock object, you should look into
// the low-level API available in the `simulacrum` crate.
//
// You can see that & and &mut parameters are mocked as *const and *mut. Also note
// that the *mut parameter uses `was_called!()` with a `()` return type and
// `.returning()` to have its return behavior specified.

extern crate simulacrum_macros;

use simulacrum_macros::*;

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

create_mock_struct! {
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
        was_called!(self, "foo")
    }

    fn bar(&mut self) {
        was_called!(self, "bar")
    }

    fn goop(&mut self, flag: bool) -> u32 {
        was_called!(self, "goop", (flag: bool) -> u32)
    }

    fn zing(&self, first: i32, second: bool) {
        was_called!(self, "zing", (first: i32, second: bool))
    }

    fn boop(&self, name: &'static str) {
        was_called!(self, "boop", (name: &'static str))
    }

    fn store(&self, val: &i64) {
        was_called!(self, "store", (val: *const i64))
    }

    fn toggle(&self, bit: &mut bool) {
        was_called!(self, "toggle", (bit: *mut bool))
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
                                   .modifying(|&mut arg| { unsafe { *arg.as_mut().unwrap() = false } });

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