//! Tests integration with some features of `thiserror`.

use locate_error::Locate;
use locate_error::Location;
use thiserror::Error;

/// Test error logging with the error and location
#[derive(Error, Debug, Locate)]
pub enum OuterError {
    // Tuple variant
    #[error("Inner error occurred: {0} at {1}")]
    Inner(#[locate_from] InnerError, Location),

    // Struct variant
    #[error("{inner} at {location}")]
    Inner2 {
        #[locate_from]
        inner: InnerError2,
        location: Location,
    },

    // Variants to illustrate `Locate` does not affect certain other thiserror features
    #[error(transparent)]
    Transparent(#[from] TransparentError),

    #[error("Source error: {source}")]
    Source {
        #[source]
        source: SourceError,
    },
}

#[derive(Error, Debug, Locate)]
pub enum InnerError {
    #[error("Inner error occurred: {0} at {1}")]
    Error(#[locate_from] String, Location),
}

#[derive(Error, Debug, Locate)]
pub enum InnerError2 {
    #[error("Inner error 2 occurred: {0} at {1}")]
    Error(#[locate_from] String, Location),
}

#[derive(Error, Debug)]
#[error("Error message")]
pub struct TransparentError {
    data: u32,
}

#[derive(Error, Debug)]
#[error("Error message")]
pub struct SourceError {
    data: u32,
}

fn main() {
    let error_message = "test".to_string();
    let line = line!();
    let inner_error: InnerError = error_message.into();
    assert!(matches!(inner_error, InnerError::Error(_, _)));
    assert!(inner_error.to_string().contains("Inner error occurred:"));
    let InnerError::Error(_, location) = &inner_error;
    assert_eq!(location.line, line + 1);
    let line2 = line!();
    let outer_error: OuterError = inner_error.into();
    assert!(matches!(outer_error, OuterError::Inner(_, _)));
    assert!(outer_error.to_string().contains("Inner error occurred:"));
    if let OuterError::Inner(_, location) = &outer_error {
        assert_eq!(location.line, line2 + 1);
    }

    let error_message = "test".to_string();
    let line3 = line!();
    let inner_error2: InnerError2 = error_message.into();
    assert!(matches!(inner_error2, InnerError2::Error(_, _)));
    assert!(inner_error2.to_string().contains("Inner error 2 occurred:"));
    let InnerError2::Error(_, location) = &inner_error2;
    assert_eq!(location.line, line3 + 1);

    let line4 = line!();
    let outer_error2: OuterError = inner_error2.into();
    assert!(matches!(outer_error2, OuterError::Inner2 { .. }));
    assert!(outer_error2.to_string().contains("Inner error 2 occurred:"));
    if let OuterError::Inner2 { inner, location } = &outer_error2 {
        assert!(inner.to_string().contains("Inner error 2 occurred"));
        assert_eq!(location.line, line4 + 1);
    }
}
