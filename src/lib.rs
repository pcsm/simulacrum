use std::collections::HashMap;

pub struct TrackedMethod {
    calls_exact: Option<i64>,
    name: String
}

impl TrackedMethod {
    pub fn new(name: String) -> Self {
        TrackedMethod {
            calls_exact: None,
            name 
        }
    }

    pub fn called_times(&mut self, calls: i64) {
        self.calls_exact = Some(calls)
    }

    pub fn called(&mut self) {
        if let Some(calls) = self.calls_exact {
            self.calls_exact = Some(calls - 1)
        }
    }

    pub fn verify(&self) {
        match self.calls_exact {
            Some(x) if x < 0 => panic!("{} was called {} times more than expected", self.name, x.abs()),
            Some(x) if x > 0 => panic!("{} was called {} times fewer than expected", self.name, x),
            _ => { }
        }
    }
}

pub struct ExpectationStore {
    inner: HashMap<u64, TrackedMethod>
}

impl ExpectationStore {
    pub fn new() -> Self {
        ExpectationStore {
            inner: HashMap::new()
        }
    }

    pub fn verify(&self) {
        for (_, exp) in self.inner.iter() {
            exp.verify();
        }
    }

    pub fn called(&mut self, key: u64) {
        if self.is_tracked(&key) {
            self.inner.get_mut(&key).unwrap().called();
        }
    }

    pub fn is_tracked(&self, key: &u64) -> bool {
        self.inner.contains_key(key)
    }

    pub fn track_method(&mut self, key: u64) -> &mut TrackedMethod {
        self.inner.entry(key).or_insert_with(|| TrackedMethod::new("XXXXX".to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
