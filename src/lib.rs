//! A cross-platform Python module, built with Rust, for obtaining comprehensive system battery information
//! including status, capacity, and temperature.

mod battery;
mod measurement;
mod enums;

use pyo3::prelude::*;
use battery::Battery;
use measurement::Measurement;
use enums::{TimeFormat, TempUnit};

/// The `batteryinfo` module provides classes and functions to interact with system batteries.
///
/// This module includes the following classes:
/// - `Battery`: Represents a system battery with properties like charge, voltage, and temperature.
/// - `Measurement`: Represents a measurement with a value, units, and precision.
/// - `TimeFormat`: Enum representing the format for displaying time.
/// - `TempUnit`: Enum representing the unit for displaying temperature.
#[pymodule]
fn batteryinfo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Battery>()?;
    m.add_class::<Measurement>()?;
    m.add_class::<TimeFormat>()?;
    m.add_class::<TempUnit>()?;
    Ok(())
}