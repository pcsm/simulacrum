use super::super::Validator;

/// A closure that will be called with the parameters to validate that they 
/// conform to the requirements.
pub struct Check<I>(Box<FnMut(&I) -> bool>);

pub fn passes<I, F>(closure: F) -> Check<I> where
    F: FnMut(&I) -> bool + 'static
{
    Check(Box::new(closure))
}

impl<I> Validator<I> for Check<I> {
    fn validate(&mut self, param: &I) -> bool {
        (self.0)(param)
    }

     fn print(&self) -> String {
        "Passes Closure".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        let mut c = passes(|arg: &i32| *arg == 555);
        let v: i32 = 555;
        assert!(c.validate(&v));
    }
}