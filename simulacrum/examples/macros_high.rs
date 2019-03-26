// This is the highest-level macro available in stable Simulacrum.
//
// It creates a Mock struct and impls a Trait - all you have to do is copy over
// the trait interface and annotate it.
//
// Note that if you want additional control, like not mocking certain parameters,
// you should use the mid-level macros shown in the `macros_mid.rs` example. For
// even more control, you can use the `simulacrum` crate directly.

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

    // Unsafe
    unsafe fn ohno(&self);
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

        expect_toggle("toggle"):
        fn toggle(&self, bit: &mut bool);

        expect_ohno("ohno"):
        unsafe fn ohno(&self);
    }
}

fn main() {
    // Create a mock object
    let mut m = CoolTraitMock::new();

    // Set up expectations for it
    m.expect_bar().called_never();
    m.expect_foo().called_once();
    m.then()
        .expect_goop()
        .called_once()
        .with(true)
        .returning(|_| 5);
    m.then()
        .expect_zing()
        .called_once()
        .with(params!(13, false));
    m.expect_boop().called_times(2);
    m.expect_store().called_once().with(deref(777));
    m.expect_toggle()
        .called_once()
        .with(deref(true))
        .modifying(|&mut arg| unsafe { *arg = false });
    m.expect_ohno().called_once();

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
    unsafe {
        m.ohno();
    }

    // When the mock object is dropped, its expectations will be evaluated
}
