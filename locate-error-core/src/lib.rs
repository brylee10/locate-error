/// Represents the location in a file, used for error reporting
#[derive(Debug)]
pub struct Location {
    file: String,
    line: u32,
    column: u32,
}

#[allow(clippy::new_without_default)]
impl Location {
    #[track_caller]
    pub fn new() -> Self {
        location!()
    }

    pub fn file(&self) -> &str {
        &self.file
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn column(&self) -> u32 {
        self.column
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Utility to get the location of the caller
#[macro_export]
macro_rules! location {
    () => {{
        let caller = ::core::panic::Location::caller();
        $crate::Location {
            file: caller.file().to_string(),
            line: caller.line(),
            column: caller.column(),
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location() {
        let file = file!();
        // Hard coded location of where `location!()` is called
        let column = 24;
        let line = line!();
        let location = location!();
        assert!(matches!(
            location,
            Location {
                file: _,
                line: _,
                column: _
            }
        ));
        assert_eq!(location.file(), file);
        assert_eq!(location.line(), line + 1);
        assert_eq!(location.column(), column);

        let column = 24;
        let line = line!();
        let location = Location::new();
        assert!(matches!(
            location,
            Location {
                file: _,
                line: _,
                column: _
            }
        ));
        assert_eq!(location.file(), file);
        assert_eq!(location.line(), line + 1);
        assert_eq!(location.column(), column);
    }
}
