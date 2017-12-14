// This is the highest-level macro in Simulacrum.
//
// It creates a Mock struct and impls a Trait - all you have to do is copy over
// the trait interface and annotate it.
//
// Note that if you want additional control, like not mocking certain parameters,
// you should use the mid-level macros shown in the `mid_level.rs` example.

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
    impl CoolTrait for CoolTraitMock (self) {
        expect_foo("foo"):
        fn foo(&self);

        expect_bar("bar"):
        fn bar(&mut self);

        expect_goop("goop"):
        fn goop(&mut self, flag: bool) -> u32;

        expect_zing("zing"):
        fn zing(&self, first: i32, second: bool);

        // &'static params are a special case - other lifetimes can't be mocked.
        expect_boop("boop"):
        fn boop(&self, name: &'static str);

        // & params are mocked as *const and &mut are mocked as *mut.
        expect_store("store"):
        fn store(&self, val: &i64);

        // If we want to modify a &mut param, we need to have `-> ()` on the end
        // to indicate that a return behavior should be specified.
        //
        // You can see how we've modified the *mut using the `.returning()` call
        // in when setting up the expectations below.
        expect_toggle("toggle"): 
        fn toggle(&self, bit: &mut bool) -> ();
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