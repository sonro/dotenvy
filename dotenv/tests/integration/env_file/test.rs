use dotenvy_test_util::TestEnv;

use crate::{test, ApiFn, TestRes};

pub fn key_set<A, E>(api_fn: A, env_file: E, key: &str, val: &str) -> TestRes
where
    A: ApiFn,
    E: Into<Vec<u8>>,
{
    let testenv = TestEnv::new_with_env_file(env_file)?;
    test::key_set(api_fn, &testenv, key, val)
}

pub fn key_unset<A, E>(api_fn: A, env_file: E, key: &str) -> TestRes
where
    A: ApiFn,
    E: Into<Vec<u8>>,
{
    let testenv = TestEnv::new_with_env_file(env_file)?;
    test::key_unset(api_fn, &testenv, key)
}
