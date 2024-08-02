use super::test;
use crate::{ApiFn, TestRes};

const KEY: &str = "TEST_COMMENT_KEY";
const VAL: &str = "comment_val";
const KEYVAL: &str = "TEST_COMMENT_KEY=comment_val";

pub fn single_var(api_fn: impl ApiFn) -> TestRes {
    test::key_set(api_fn, KEYVAL, KEY, VAL)
}

macro_rules! tests {
    ($api_fn:expr) => {
        #[test]
        fn single_var() -> $crate::TestRes {
            $crate::env_file::basic::single_var($api_fn)
        }
    };
}
pub(crate) use tests;
