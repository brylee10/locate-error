//! Enums which use the `Locate` should have at least one variant with a field marked with the #[locate_from] attribute.
//! The fields which have this attribute should be a two element tuple, with the first element being the source and the second element being the location.

use locate_error::Locate;
use locate_error::Location;

// Too many fields
#[derive(Locate)]
enum TestEnum {
    Variant1(#[locate_from] String, Location, String),
}

// No location field
#[derive(Locate)]
enum TestEnum2 {
    Variant1(#[locate_from] String),
}

// No locate_from attribute
#[derive(Locate)]
enum TestEnum3 {
    Variant1(String, Location),
    Variant2(String),
}

// Too many fields with locate_from attribute
#[derive(Locate)]
enum TestEnum4 {
    Variant1(#[locate_from] String, Location, #[locate_from] u32),
}

// No location field
#[derive(Locate)]
enum TestEnum5 {
    Variant1(#[locate_from] String, u32),
}

// Error location should be on the variant name, not the attribute
#[derive(Debug, Locate, thiserror::Error)]
enum TestEnum6 {
    #[error(transparent)]
    Variant1(#[locate_from] SomeError),
}

#[derive(Debug, thiserror::Error)]
#[error("Error message")]
struct SomeError {
    message: String,
}

fn main() {}
