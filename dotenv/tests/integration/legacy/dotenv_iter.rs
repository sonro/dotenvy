mod env_file {
    use crate::legacy::IterApiFn;

    pub struct DotenvIter(IterApiFn);

    impl DotenvIter {
        fn new() -> Self {
            Self(IterApiFn::new())
        }
    }

    impl crate::ApiFn for DotenvIter {
        fn run(&mut self) {
            let iter = dotenvy::dotenv_iter().unwrap();
            self.0.process(iter);
        }

        fn run_err(&mut self) -> dotenvy::Error {
            match dotenvy::dotenv_iter() {
                Err(err) => err,
                Ok(iter) => self.0.process_err(iter),
            }
        }

        fn assert_var(&self, key: &str, exepcted: &str) {
            self.0.assert_var(key, exepcted);
        }

        fn assert_var_unset(&self, key: &str) {
            self.0.assert_var_unset(key);
        }
    }

    crate::env_file::tests!(DotenvIter::new());
}
