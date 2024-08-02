use dotenvy_test_util::{test_in_env, TestEnv};

use crate::{ApiFn, TestRes};

pub fn key_set<A>(mut api_fn: A, testenv: &TestEnv, key: &str, val: &str) -> TestRes
where
    A: ApiFn,
{
    test_in_env(testenv, || {
        api_fn.run();
        api_fn.assert_var(key, val);
    })
}

pub fn key_unset<A>(mut api_fn: A, testenv: &TestEnv, key: &str) -> TestRes
where
    A: ApiFn,
{
    test_in_env(testenv, || {
        api_fn.run();
        api_fn.assert_var_unset(key);
    })
}
