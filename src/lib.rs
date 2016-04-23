#![doc(html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk-v2.png",
       html_favicon_url = "https://www.rust-lang.org/favicon.ico",
       html_root_url = "https://wisagan.github.io/simple-error/simple_error/")]
#![deny(missing_docs)]
//! A simple error type backed by a string.
//!
//! This crate provides a `SimpleError` type, which implements `std::error::Error`. The underlying
//! is a `String` as the error message.
//!
//! It should be used when all you care about is an error string.
//!
//! It should not be used when you want to programmatically handle different kinds of an error.

use std::fmt;

/// A type that represents a simple error.
///
/// This type uses a `String` to store the error string, and it implements `std::error::Error`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleError {
    err: String,
}

impl SimpleError {
    /// Creates a new simple error.
    ///
    /// This function can take any type that implements `Into<String>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    ///
    /// // errors can be created from `str`
    /// let err = SimpleError::new("an error from str");
    ///
    /// // errors can also be created from `String`
    /// let err = SimpleError::new(String::from("an error from String"));
    /// ```
    pub fn new<T: Into<String>>(t: T) -> SimpleError {
        SimpleError{ err: t.into() }
    }

    /// Creates a new simple error from another error.
    ///
    /// This function can take any type that implements `std::error::Error`.
    /// The error string will be the `Display` of the `std::error::Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    /// use std::io;
    ///
    /// // errors can be created from `io::Error`
    /// let err = SimpleError::from(io::Error::new(io::ErrorKind::Other, "oh no"));
    /// assert_eq!("oh no", format!("{}", err));
    /// ```
    pub fn from<T: std::error::Error>(t: T) -> SimpleError {
        SimpleError{ err: format!("{}", t) }
    }

    /// Creates a new simple error from a string with another error.
    ///
    /// This function takes a string as error and a type that implements `std::error::Error` as
    /// reason.
    /// The error string will be the `Display` of the `std::error::Error` prefixed with the string
    /// and ", ".
    ///
    /// # Examples
    ///
    /// ```
    /// use self::simple_error::SimpleError;
    ///
    /// let err = SimpleError::with("cannot turn on tv", SimpleError::new("remote not found"));
    /// assert_eq!("cannot turn on tv, remote not found", format!("{}", err));
    /// ```
    pub fn with<T: std::error::Error>(e: &str, t: T) -> SimpleError {
        SimpleError{ err: format!("{}, {}", e, t) }
    }
}

// TODO: implement From<T> where T: std::error::Error when specialization lands, and remove
// inherent from function.

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.err.fmt(f)
    }
}

impl std::error::Error for SimpleError {
    fn description(&self) -> &str {
        &self.err
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleError;
    use std::error::Error;
    use std::io;

    #[test]
    fn new_from_string() {
        let err = SimpleError::new(String::from("an error from String"));
        assert_eq!("an error from String", format!("{}", err));
        assert_eq!("an error from String", err.description());
    }

    #[test]
    fn new_from_str() {
        let err = SimpleError::new("an error from str");
        assert_eq!("an error from str", format!("{}", err));
    }

    #[test]
    fn from_io_error() {
        let err = SimpleError::from(io::Error::new(io::ErrorKind::Other, "oh no"));
        assert_eq!("oh no", format!("{}", err));
    }
}
