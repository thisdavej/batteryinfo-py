use pyo3::prelude::*;
use std::fmt;

/// Represents a measurement with a value, units, and precision.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Measurement {
    /// The value of the measurement.
    pub value: f32,
    /// The units of the measurement.
    pub units: String,
    /// The number of decimal places to display.
    pub decimals: usize,
}

impl Measurement {
    /// A private helper method to format the measurement as a string.
    fn format_measurement(&self) -> String {
        let formatted_value = format!("{:.precision$}", self.value, precision = self.decimals);
        if self.units == "%" {
            format!("{}{}", formatted_value, self.units)
        } else {
            format!("{} {}", formatted_value, self.units)
        }
    }
}

#[pymethods]
impl Measurement {
    /// Creates a new `Measurement` instance.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the measurement.
    /// * `units` - The units of the measurement.
    /// * `decimals` - The number of decimal places to display.
    #[new]
    pub fn new(value: f32, units: String, decimals: usize) -> Self {
        Self {
            value,
            units,
            decimals,
        }
    }

    /// Returns the value of the measurement.
    #[getter]
    fn value(&self) -> PyResult<f32> {
        Ok(self.value)
    }

    /// Returns the units of the measurement.
    #[getter]
    fn units(&self) -> PyResult<String> {
        Ok(self.units.clone())
    }

    /// Returns a formatted string representation of the measurement.
    ///
    /// If the unit is percent, the value and unit are formatted without a space between them.
    pub fn formatted(&self) -> PyResult<String> {
        Ok(self.format_measurement())
    }

    /// Returns a string representation of the measurement.
    ///
    /// This method is used by Python's `repr()` function.
    fn __repr__(&self) -> PyResult<String> {
        self.formatted()
    }
}

impl fmt::Display for Measurement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_measurement())
    }
}
