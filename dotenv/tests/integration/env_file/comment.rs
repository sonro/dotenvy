use super::test;
use crate::{ApiFn, TestRes};

const KEY: &str = "TEST_COMMENT_KEY";
const VAL: &str = "comment_val";
const KEYVAL: &str = "TEST_COMMENT_KEY=comment_val";

pub fn ignore_var_no_ws(api_fn: impl ApiFn) -> TestRes {
    test_key_unset(api_fn, &format!("#{KEYVAL}"))
}

pub fn ignore_var_space(api_fn: impl ApiFn) -> TestRes {
    test_key_unset(api_fn, &format!("# {KEYVAL}"))
}

pub fn ignore_var_tab(api_fn: impl ApiFn) -> TestRes {
    test_key_unset(api_fn, &format!("#\t{KEYVAL}"))
}

pub fn ignore_var_leading_space(api_fn: impl ApiFn) -> TestRes {
    test_key_unset(api_fn, &format!(" # {KEYVAL}"))
}

pub fn ignore_var_leading_tab(api_fn: impl ApiFn) -> TestRes {
    test_key_unset(api_fn, &format!("\t# {KEYVAL}"))
}

pub fn assign_space_hash(api_fn: impl ApiFn) -> TestRes {
    test_key_set(api_fn, &format!("{KEYVAL} #comment"))
}

fn test_key_set(api_fn: impl ApiFn, env_file: &str) -> TestRes {
    test::key_set(api_fn, env_file, KEY, VAL)
}

fn test_key_unset(api_fn: impl ApiFn, env_file: &str) -> TestRes {
    test::key_unset(api_fn, env_file, KEY)
}

macro_rules! tests {
    ($api_fn:expr) => {
        #[test]
        fn ignore_var_no_ws() -> $crate::TestRes {
            $crate::env_file::comment::ignore_var_no_ws($api_fn)
        }

        #[test]
        fn ignore_var_space() -> $crate::TestRes {
            $crate::env_file::comment::ignore_var_space($api_fn)
        }

        #[test]
        fn ignore_var_tab() -> $crate::TestRes {
            $crate::env_file::comment::ignore_var_tab($api_fn)
        }

        #[test]
        fn ignore_var_leading_space() -> $crate::TestRes {
            $crate::env_file::comment::ignore_var_leading_space($api_fn)
        }

        #[test]
        fn ignore_var_leading_tab() -> $crate::TestRes {
            $crate::env_file::comment::ignore_var_leading_tab($api_fn)
        }

        #[test]
        fn assign_space_hash() -> $crate::TestRes {
            $crate::env_file::comment::assign_space_hash($api_fn)
        }
    };
}
pub(crate) use tests;
