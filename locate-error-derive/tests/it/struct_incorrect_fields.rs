//! Only a source and a location field should be present in a struct.
//! Otherwise, raise a compile error.

use locate_error::Locate;
use locate_error::Location;

#[derive(Locate)]
pub struct ExtraFields {
    #[locate_from]
    // This source field can be named anything
    pub inner: Inner,
    pub location: Location,
    pub extra_field: String,
}

#[derive(Locate)]
pub struct MissingLocation {
    #[locate_from]
    pub inner: Inner,
    pub extra_field: String,
}

#[derive(Locate)]
pub struct MissingSource {
    pub location: Location,
    pub extra_field: String,
}

#[derive(Locate)]
pub struct MultipleSourceFields {
    #[locate_from]
    pub inner: Inner,
    pub location: Location,
    #[locate_from]
    pub extra_field: String,
}

pub struct Inner {
    pub field: String,
}

fn main() {}
