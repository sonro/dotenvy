use std::collections::HashMap;

pub struct IterApiFn {
    vars: HashMap<String, String>,
}

impl IterApiFn {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn process_err<R>(iter: dotenvy::Iter<R>) -> dotenvy::Error
    where
        R: std::io::Read,
    {
        for res in iter {
            if let Err(err) = res {
                return err;
            }
        }
        panic!("expected an error, but got none");
    }

    pub fn process<R>(&mut self, iter: dotenvy::Iter<R>)
    where
        R: std::io::Read,
    {
        for res in iter {
            match res {
                Ok((key, val)) => {
                    self.vars.insert(key, val);
                }
                Err(err) => {
                    panic!("unexpected error: {}", err);
                }
            }
        }
    }

    pub fn assert_var(&self, key: &str, exepcted: &str) {
        let actual = self
            .vars
            .get(key)
            .unwrap_or_else(|| panic!("expected {} to be loaded", key));

        assert_eq!(
            exepcted, actual,
            "unexpected value for {key}:\n  EXPECTED: {exepcted}\n    ACTUAL: {actual}\n",
        );
    }

    pub fn assert_var_unset(&self, key: &str) {
        let actual = self.vars.get(key);
        assert!(
            actual.is_none(),
            "unexpected {key} value: {}",
            actual.unwrap()
        );
    }
}
