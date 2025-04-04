//! Test invalid enums with struct variants

use locate_error::Locate;
use locate_error::Location;

// Too many fields
#[derive(Locate)]
enum TestEnum {
    Variant1 {
        #[locate_from]
        source: String,
        location: Location,
        extra: String,
    },
}

// No location field
#[derive(Locate)]
enum TestEnum2 {
    Variant1 {
        #[locate_from]
        source: String,
        // Missing location field
    },
}

// No locate_from attribute
#[derive(Locate)]
enum TestEnum3 {
    Variant1 {
        source: String, // Missing #[locate_from]
        location: Location,
    },
    Variant2 {
        source: String,
    },
}

// Too many fields with locate_from attribute
#[derive(Locate)]
enum TestEnum4 {
    Variant1 {
        #[locate_from]
        source: String,
        location: Location,
        #[locate_from]
        extra: u32,
    },
}

// No location field
#[derive(Locate)]
enum TestEnum5 {
    Variant1 {
        #[locate_from]
        source: String,
        value: u32, // This should be a location field
    },
}

fn main() {}
