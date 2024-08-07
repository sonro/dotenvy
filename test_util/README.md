# dotenvy test util

This is an internal package used for testing dotenvy.

## Why

Eases the annoyance of setting up custom `.env` files, managing existing
environment variables, and running multiple tests at once.

## How

By setting up a `TestEnv`, and running a closure via `test_in_env`.

**Before** executing the closure, the `TestEnv` will:

- Create a temporary directory
- Lock the environment from other tests
- Store the existing environment variables
- Add any custom env_vars to the environment
- Create any custom env files in the temporary directory

**In the closure** you can use the assertion functions to test the environment.

**After** executing the closure, the `TestEnv` will:

- Remove the temporary directory
- Restore the environment variables to the original state
- Unlock the environment

See the API docs for more details. For now, they must be built locally with
`cargo doc`.

### Commented example

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv_override;

const EXISTING_KEY: &str = "TEST_KEY";
const EXISTING_VAL: &str = "test_val";
const OVERRIDING_VAL: &str = "overriding_val";

#[test]
fn dotenv_override_existing_key() -> Result<(), Error> {
    // setup testing environment
    let mut testenv = TestEnv::init()?;

    // with an existing environment variable
    testenv.add_env_var(EXISTING_KEY, EXISTING_VAL)?;

    // with an env file that overrides it
    testenv.add_env_file(
        ".env",
        create_custom_env_file(&[(EXISTING_KEY, OVERRIDING_VAL)]),
    )?;

    // execute a closure in the testing environment
    test_in_env(&testenv, || {
        dotenv_override().expect(".env should be loaded");
        assert_env_var(EXISTING_KEY, OVERRIDING_VAL);
    })
    // any changes to environment variables will be reset for other tests
}
```

### Customised Envfile

Use the `EnvFileContents` to manipulate the content of an env file. Useful
for byte-order-mark(BOM) testing, and other edge cases.

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv;

#[test]
fn comments_ignored_in_utf8bom_env_file() -> Result<(), Error> {
    let mut efc = EnvFileContents::new();
    efc.push_bytes(&[0xEF, 0xBB, 0xBF]);
    efc.push_str("# TEST_KEY=TEST_VAL\n");

    let testenv = TestEnv::init_with_env_file(efc)?;

    test_in_env(&testenv, || {
        dotenv().expect(".env should be loaded");
        assert_env_var_unset("TEST_KEY");
    })
}
```

Or use anything that can be converted to a `Vec<u8>` if your env_file is
simple.

```rust
use dotenvy_test_util::*;
use dotenvy::dotenv;

#[test]
fn comments_ignored() -> Result<(), Error> {
    let env_file = "# TEST_KEY=TEST_VAL\n";

    let testenv = TestEnv::init_with_env_file(env_file)?;

    test_in_env(&testenv, || {
        dotenv().expect(".env should be loaded");
        assert_env_var_unset("TEST_KEY");
    })
}
```

