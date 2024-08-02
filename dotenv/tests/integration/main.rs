#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

/// legacy API functions
mod legacy;

/// different env file test-cases
mod env_file;

/// standard env vars and env file
mod default;

/// tests in ``TestEnv``
mod test;

type TestRes<T = ()> = Result<T, dotenvy_test_util::Error>;

pub trait ApiFn {
    fn run(&mut self);

    fn run_err(&mut self) -> dotenvy::Error;

    fn assert_var(&self, key: &str, expected: &str) {
        dotenvy_test_util::assert_env_var(key, expected);
    }

    fn assert_var_unset(&self, key: &str) {
        dotenvy_test_util::assert_env_var_unset(key);
    }
}
