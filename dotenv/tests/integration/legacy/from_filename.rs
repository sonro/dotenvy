mod env_file {
    use crate::default;

    pub struct FromFilename;

    impl crate::ApiFn for FromFilename {
        fn run(&mut self) {
            dotenvy::from_filename(default::ENV_FILE_PATH).unwrap();
        }

        fn run_err(&mut self) -> dotenvy::Error {
            dotenvy::from_filename(default::ENV_FILE_PATH).unwrap_err()
        }
    }

    crate::env_file::tests!(FromFilename);
}
