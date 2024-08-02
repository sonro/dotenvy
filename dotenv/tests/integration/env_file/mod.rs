pub mod basic;
pub mod comment;

mod test;

macro_rules! tests {
    ($api_fn:expr) => {
        mod basic {
            use super::*;
            $crate::env_file::basic::tests!($api_fn);
        }
        mod comment {
            use super::*;
            $crate::env_file::comment::tests!($api_fn);
        }
    };
}
pub(crate) use tests;
