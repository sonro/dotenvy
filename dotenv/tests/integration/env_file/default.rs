use dotenvy_test_util::TestEnv;

use crate::TestRes;

pub const EXISTING_KEY: &str = "EXISTING_KEY";
pub const EXISTING_VAL: &str = "EXISTING_VALUE";

pub const ENV_FILE_PATH: &str = ".env";

pub fn testenv_with_env_file(env_file: impl Into<Vec<u8>>) -> TestRes<TestEnv> {
    let mut testenv = TestEnv::new_with_env_file(env_file)?;
    testenv.add_env_var(EXISTING_KEY, EXISTING_VAL)?;
    Ok(testenv)
}
