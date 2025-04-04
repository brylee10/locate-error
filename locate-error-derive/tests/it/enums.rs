//! Tests standalone `Locate` macro usage on enums.
//! Includes `From` impls on nested types and parsing of location.

use locate_error::Locate;
use locate_error::Location;

#[derive(Locate)]
pub enum TestError {
    Simple(#[locate_from] String, Location),

    Complex {
        #[locate_from]
        source: std::io::Error,
        location: Location,
    },
}

fn main() {
    // Hard coded position of where `.into()` is called.
    let column = 54;
    let line = line!();
    let err: TestError = "error message".to_string().into();
    let this_file = file!().to_string();
    match err {
        TestError::Simple(msg, location) => {
            assert_eq!(msg, "error message");
            assert!(location.file.contains(&this_file));
            assert_eq!(location.line, line + 1);
            assert_eq!(location.column, column);
        }
        _ => panic!("Wrong variant"),
    }

    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let column = 33;
    let line = line!();
    let err: TestError = io_err.into();
    match err {
        TestError::Complex { source, location } => {
            assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
            assert!(location.file.contains(&this_file));
            assert_eq!(location.line, line + 1);
            assert_eq!(location.column, column);
        }
        _ => panic!("Wrong variant"),
    }
}
