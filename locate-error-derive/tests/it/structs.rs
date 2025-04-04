//! Tests standalone `Locate` macro usage on structs.
//! Includes `From` impls on nested types and parsing of location.

use locate_error::Locate;
use locate_error::Location;

#[derive(Locate)]
pub struct TestStruct {
    #[locate_from]
    pub inner: Inner,
    pub location: Location,
}

pub struct Inner {
    pub field: String,
}

fn main() {
    let file = file!();

    let inner = Inner {
        field: "inner".to_string(),
    };
    // Hard coded position of where `.into()` is called.
    let column = 41;
    let line = line!();
    let test_struct: TestStruct = inner.into();

    assert_eq!(test_struct.inner.field, "inner");
    assert_eq!(test_struct.location.file, file);
    assert_eq!(test_struct.location.line, line + 1);
    assert_eq!(test_struct.location.column, column);
}
