//! The checks that tests make on what the library returns from a simulated
//! device.

use tapo::TapoResponseError;

/// Asserts that `login` failed as unauthorized, with the error `kind`.
pub fn assert_unauthorized<T>(login: Result<T, tapo::Error>, kind: &str) {
    let Err(error) = login else {
        panic!("the login succeeded");
    };

    assert!(
        matches!(
            &error,
            tapo::Error::Tapo(TapoResponseError::Unauthorized { kind: actual, .. }) if *actual == kind
        ),
        "{error:?}"
    );
}
