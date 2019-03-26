use simulacrum_shared::Validator;

use std::marker::PhantomData;

/// Parameter(s) must equal the provided value.
pub struct Deref<I, V>(V, PhantomData<I>)
where
    V: Validator<I>;

pub fn deref<I, V>(validator: V) -> Deref<I, V>
where
    V: Validator<I>,
{
    Deref(validator, PhantomData)
}

impl<I, V> Validator<*mut I> for Deref<I, V>
where
    V: Validator<I>,
{
    fn validate(&mut self, param: &*mut I) -> bool {
        unsafe { self.0.validate(&*param.as_mut().unwrap()) }
    }

    fn print(&self) -> String {
        format!("&mut {}", self.0.print()).to_owned()
    }
}

impl<I, V> Validator<*const I> for Deref<I, V>
where
    V: Validator<I>,
{
    fn validate(&mut self, param: &*const I) -> bool {
        unsafe { self.0.validate(&*param.as_ref().unwrap()) }
    }

    fn print(&self) -> String {
        format!("& {}", self.0.print()).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_const() {
        let mut c = deref(888);
        let v = &888 as *const i32;
        assert!(c.validate(&v));
    }

    #[test]
    #[should_panic]
    fn test_validate_const_fail() {
        let mut c = deref(555);
        let v = &888 as *const i32;
        assert!(c.validate(&v));
    }

    #[test]
    fn test_validate_mut() {
        let mut c = deref(888);
        let v = &mut 888 as *mut i32;
        assert!(c.validate(&v));
    }

    #[test]
    #[should_panic]
    fn test_validate_mut_fail() {
        let mut c = deref(555);
        let v = &mut 888 as *mut i32;
        assert!(c.validate(&v));
    }
}
