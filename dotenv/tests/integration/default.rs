#![allow(dead_code)]

use crate::TestRes;
use dotenvy_test_util::{assert_env_var, test_in_env, TestEnv};

pub const FILE_KEY: &str = "DEFULT_FILE_KEY";
pub const FILE_VAL: &str = "testvalue";

pub const EXISTING_KEY: &str = "DEFAULT_EXISTING_KEY";
pub const EXISTING_VAL: &str = "from_env";
pub const OVERRIDE_VAL: &str = "from_file";

pub const ENV_FILE: &str = "DEFAULT_FILE_KEY=testvalue\nDEFAULT_EXISTING_KEY=from_file";

pub const ENV_FILE_PATH: &str = ".env";

pub fn testenv() -> TestRes<TestEnv> {
    let mut testenv = TestEnv::new()?;
    testenv.add_env_var(EXISTING_KEY, EXISTING_VAL)?;
    testenv.add_env_file(ENV_FILE_PATH, ENV_FILE)?;
    Ok(testenv)
}

pub fn test<F>(f: F) -> TestRes
where
    F: FnOnce(),
{
    let testenv = testenv()?;
    test_in_env(&testenv, f)
}

pub fn assert_vars() {
    assert_env_var(FILE_KEY, FILE_VAL);
    assert_env_var(EXISTING_KEY, EXISTING_VAL);
}

pub fn assert_vars_override() {
    assert_env_var(FILE_KEY, FILE_VAL);
    assert_env_var(EXISTING_KEY, OVERRIDE_VAL);
}
