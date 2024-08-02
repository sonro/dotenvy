mod env_file {
    use crate::env_file::default;
    use crate::legacy::IterApiFn;

    pub struct FromFilenameIter(IterApiFn);

    impl FromFilenameIter {
        pub fn new() -> Self {
            Self(IterApiFn::new())
        }
    }

    impl crate::ApiFn for FromFilenameIter {
        fn run(&mut self) {
            let iter = dotenvy::from_filename_iter(default::ENV_FILE_PATH).unwrap();
            self.0.process(iter);
        }

        fn run_err(&mut self) -> dotenvy::Error {
            match dotenvy::from_filename_iter(default::ENV_FILE_PATH) {
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

    crate::env_file::tests!(FromFilenameIter::new());
}
