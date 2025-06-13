#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;


pub mod sensors;
/// A sensor reading
#[derive(Debug, Clone, Copy)]
pub struct Reading {
    humidity: f32,
    temperature: f32,
}

impl Reading {
    /// Returns the ambient humidity, as a percentage value from 0.0 to 100.0
    pub fn humidity(&self) -> f32 {
        self.humidity
    }

    /// Returns the ambient temperature, in degrees Celsius
    pub fn temperature(&self) -> f32 {
        self.temperature
    }
}

/// A type detailing various errors the DHT sensor can return
#[derive(Debug, Clone)]
pub enum DhtError<HE> {
    /// The DHT sensor was not found on the specified GPIO
    NotPresent,
    /// The checksum provided in the DHT sensor data did not match the checksum of the data itself (expected, calculated)
    ChecksumMismatch(u8, u8),
    /// The seemingly-valid data has impossible values (e.g. a humidity value less than 0 or greater than 100)
    InvalidData,
    /// The read timed out
    Timeout,
    /// Received a low-level error from the HAL while reading or writing to pins
    PinError(HE),
}

impl<HE> From<HE> for DhtError<HE> {
    fn from(error: HE) -> Self {
        DhtError::PinError(error)
    }
}

impl<HE: fmt::Debug> fmt::Display for DhtError<HE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DhtError::*;
        match self {
            NotPresent => write!(f, "DHT device not found"),
            ChecksumMismatch(expected, calculated) => write!(
                f,
                "Data read was corrupt (expected checksum {:x}, calculated {:x})",
                expected, calculated
            ),
            InvalidData => f.write_str("Received data is out of range"),
            Timeout => f.write_str("Timed out waiting for a read"),
            PinError(err) => write!(f, "HAL pin error: {:?}", err),
        }
    }
}

impl<HE: fmt::Debug> core::error::Error for DhtError<HE> {}

pub trait DhtSensor<HE> {
    /// Reads data from the sensor and returns a `Reading`
    fn read(&mut self) -> Result<Reading, DhtError<HE>>;
}

