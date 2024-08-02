mod env_file {
    use std::{fs, io};

    use crate::env_file::default;

    pub struct FromRead;

    impl crate::ApiFn for FromRead {
        fn run(&mut self) {
            dotenvy::from_read(reader()).unwrap();
        }

        fn run_err(&mut self) -> dotenvy::Error {
            dotenvy::from_read(reader()).unwrap_err()
        }
    }

    fn reader() -> io::BufReader<fs::File> {
        let file = fs::File::open(default::ENV_FILE_PATH).expect("failed to open .env file");
        io::BufReader::new(file)
    }

    crate::env_file::tests!(FromRead);
}
