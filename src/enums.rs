use pyo3::prelude::*;

/// Represents the format for displaying time.
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeFormat {
    /// Display time in seconds.
    Seconds,
    /// Display time in minutes.
    Minutes,
    /// Display time in a human-readable format.
    Human,
}

#[pymethods]
impl TimeFormat {
    /// Returns a string representation of the time format.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}

/// Represents the unit for displaying temperature.
#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TempUnit {
    /// Display temperature in degrees Celsius.
    DegC,
    /// Display temperature in degrees Fahrenheit.
    DegF,
}

#[pymethods]
impl TempUnit {
    /// Returns a string representation of the temperature unit.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}