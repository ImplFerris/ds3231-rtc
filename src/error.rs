//! Error type definitions for the DS3231 RTC driver.
//!
//! This module defines the `Error` enum and helper functions
//! for classifying and handling DS3231-specific failures.

use rtc_hal::datetime::DateTimeError;

/// DS3231 driver errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<I2cError> {
    /// I2C communication error
    I2c(I2cError),
    /// Invalid register address
    InvalidAddress,
    /// The specified square wave frequency is not supported by the RTC
    UnsupportedSqwFrequency,
    /// Invalid date/time parameters provided by user
    DateTime(DateTimeError),
    /// Invalid Base Century (It should be either 19,20,21)
    InvalidBaseCentury,
}

// /// Converts an [`I2cError`] into an [`Error`] by wrapping it in the
// /// [`Error::I2c`] variant.
// ///
impl<I2cError> From<I2cError> for Error<I2cError> {
    fn from(value: I2cError) -> Self {
        Error::I2c(value)
    }
}

impl<I2cError> rtc_hal::error::RtcError for Error<I2cError> {
    fn kind(&self) -> rtc_hal::error::ErrorKind {
        match self {
            Error::I2c(_) => rtc_hal::error::ErrorKind::Bus,
            Error::InvalidAddress => rtc_hal::error::ErrorKind::InvalidAddress,
            Error::DateTime(_) => rtc_hal::error::ErrorKind::InvalidDateTime,
            Error::UnsupportedSqwFrequency => rtc_hal::error::ErrorKind::UnsupportedSqwFrequency,
            Error::InvalidBaseCentury => rtc_hal::error::ErrorKind::InvalidDateTime,
        }
    }
}

/// Implements [`defmt::Format`] for [`Error<I2cError>`].
///
/// This enables the error type to be formatted efficiently when logging
/// with the `defmt` framework in `no_std` environments.
///
/// The implementation is only available when the `defmt` feature is enabled
/// and requires that the underlying `I2cError` type also implements
/// [`core::fmt::Debug`].
///
/// Each variant is printed with a short, human-readable description,
/// and the `I2c` variant includes the inner I2C error.
#[cfg(feature = "defmt")]
impl<I2cError> defmt::Format for Error<I2cError>
where
    I2cError: core::fmt::Debug,
{
    fn format(&self, f: defmt::Formatter) {
        match self {
            Error::I2c(e) => {
                defmt::write!(f, "I2C communication error: {:?}", defmt::Debug2Format(e))
            }
            Error::InvalidAddress => defmt::write!(f, "Invalid NVRAM address"),
            Error::DateTime(_) => defmt::write!(f, "Invalid date/time values"),
            Error::UnsupportedSqwFrequency => defmt::write!(f, "Unsupported Square Wave Frequency"),
            Error::InvalidBaseCentury => {
                defmt::write!(f, "Base century must be 19 or greater")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rtc_hal::datetime::DateTimeError;
    use rtc_hal::error::{ErrorKind, RtcError};

    #[test]
    fn test_from_i2c_error() {
        #[derive(Debug, PartialEq, Eq)]
        struct DummyI2cError(u8);

        let e = Error::from(DummyI2cError(42));
        assert_eq!(e, Error::I2c(DummyI2cError(42)));
    }

    #[test]
    fn test_error_kind_mappings() {
        // I2c variant
        let e: Error<&str> = Error::I2c("oops");
        assert_eq!(e.kind(), ErrorKind::Bus);

        // InvalidAddress
        let e: Error<&str> = Error::InvalidAddress;
        assert_eq!(e.kind(), ErrorKind::InvalidAddress);

        // DateTime
        let e: Error<&str> = Error::DateTime(DateTimeError::InvalidDay);
        assert_eq!(e.kind(), ErrorKind::InvalidDateTime);

        // UnsupportedSqwFrequency
        let e: Error<&str> = Error::UnsupportedSqwFrequency;
        assert_eq!(e.kind(), ErrorKind::UnsupportedSqwFrequency);

        // InvalidBaseCentury
        let e: Error<&str> = Error::InvalidBaseCentury;
        assert_eq!(e.kind(), ErrorKind::InvalidDateTime);
    }
}
