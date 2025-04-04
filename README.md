# `locate-error`: A poor man's backtrace for `thiserror`
`locate-error` is a fairly minimal procedural macro which simply adds location information (file, line, column) to where errors occur wwhen the `From<Error> for OtherError` trait is called on `Error`, typically triggered on `?` when raising errors across functions. This is intended to be used with `thiserror::Error` when using the `#[from]` attribute. This is useful with nested types which derive `thiserror::Error` and you want to bubble up errors recursively, keeping location information for where each intermediate error occurred. `thiserror` does support `std::backtrace::Backtrace` but as of this writing requires a nightly compiler. 

The goal is to help users have another resource to add useful context to error messages to avoid unhelpful messages where the display/debug info of the bottom most error is bubbled up without context. 
```
$ cargo run
Error: Program ended
```
(this was created by a real program)

An output like the above is created by running the example program below but replacing `#[locate_from]` with the vanilla `#[from]` attribute and removing the associated `Location` information.

This is intended to supplement other crates such as `anyhow` which can provide context for errors. `snafu` is an alternative which has stable backtrace support.

# Example Usage
Before describing the components, an example gives more context on usage. A typical use with `thiserror` with nested errors would be:

```rust
use locate_error::Locate;
use locate_error::Location;
use locate_error::location;
use thiserror::Error;


#[derive(Error, Debug, Locate)]
pub enum OuterError {
    #[error("{0} \n\toccurred at {1}")]
    MiddleError(#[locate_from] MiddleError, Location),
}

#[derive(Error, Debug, Locate)]
#[error("{inner_error} \n\toccurred at {location}")]
pub struct MiddleError {
    #[locate_from]
    inner_error: InnerError,
    location: Location,
}

#[derive(Error, Debug)]
#[error("{message} \n\toccurred at {location}")]
pub struct InnerError {
    message: String,
    location: Location,
}

fn main() {
    let err: OuterError = raise_middle_error().unwrap_err().into();
    println!("{:}", err);
}

fn raise_middle_error() -> Result<(), MiddleError> {
    raise_inner_error()?;
    Ok(())
}

fn raise_inner_error() -> Result<(), InnerError> {
    Err(InnerError {
        message: format!("Error: Program ended"),
        location: location!(),
    })
}
```

Which outputs
```
Exception raised in a local function 
        occurred at app/src/bin/locate_error.rs:40:19 
        occurred at app/src/bin/locate_error.rs:33:5 
        occurred at app/src/bin/locate_error.rs:28:61
```

# Components
This crate introduces only a few components:
- The `Location` type which holds a file, column, and line number 
- The `Locate` derive macro which uses the `#[locate_from]` attribute to implement the `From<Inner> for Outer` trait for the modified inner error type.
- The `location` macro which returns a `Location` corresponding to the call site

Enum variants or structs that use the `#[locate_from]` attribute must also include a field of type `Location` which will be automatically populated with the location where the `From` trait is called. Since an additional field is added, `thiserror` attributes such as `#[error(transparent)]` do not work, so a display message must be provided.