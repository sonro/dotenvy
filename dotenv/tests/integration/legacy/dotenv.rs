mod env_file {
    pub struct Dotenv;

    impl crate::ApiFn for Dotenv {
        fn run(&mut self) {
            dotenvy::dotenv().unwrap();
        }

        fn run_err(&mut self) -> dotenvy::Error {
            dotenvy::dotenv().unwrap_err()
        }
    }

    crate::env_file::tests!(Dotenv);
}
