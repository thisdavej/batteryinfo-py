use battery::Manager;
use battery::units::{
    electric_potential::volt,
    energy::watt_hour,
    power::watt,
    ratio::percent,
    thermodynamic_temperature::{degree_celsius, degree_fahrenheit},
};
use human_time::ToHumanTimeString;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::time::{Duration, Instant};

use crate::enums::{TempUnit, TimeFormat};
use crate::measurement::Measurement;

/// Represents a system battery with properties like charge, voltage, and temperature.
#[pyclass]
#[derive(Debug)]
pub struct Battery {
    /// The vendor of the battery.
    pub vendor: Option<String>,
    /// The model of the battery.
    pub model: Option<String>,
    /// The serial number of the battery.
    pub serial_number: Option<String>,
    /// The technology of the battery.
    pub technology: String,
    /// The percentage of the battery that is full.
    pub percent_full: Measurement,
    /// The state of the battery (charging, discharging, etc.).
    pub state: battery::State,
    /// The capacity of the battery.
    pub capacity: Measurement,
    /// The temperature of the battery.
    pub temperature: Option<Measurement>,
    /// The cycle count of the battery.
    pub cycle_count: Option<u32>,
    /// The current energy of the battery.
    pub energy: Measurement,
    /// The full energy of the battery.
    pub energy_full: Measurement,
    /// The design energy of the battery.
    pub energy_full_design: Measurement,
    /// The energy rate of the battery.
    pub energy_rate: Measurement,
    /// The voltage of the battery.
    pub voltage: Measurement,
    /// The time to empty the battery.
    pub time_to_empty: Option<String>,
    /// The time to fully charge the battery.
    pub time_to_full: Option<String>,
    /// The format for displaying time.
    pub time_format: TimeFormat,

    /// The unit for displaying temperature.
    pub temp_unit: TempUnit,
    /// The last time the battery information was refreshed.
    last_refresh: Instant,
    /// The interval for refreshing the battery information.
    refresh_interval: Duration,
    /// The index of the battery.
    battery_index: usize,
}

impl Battery {
    /// Retrieves battery information and creates a new `Battery` instance.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the battery to retrieve.
    /// * `time_format` - The format for displaying time.
    /// * `temp_unit` - The unit for displaying temperature.
    /// * `refresh_interval` - The interval for refreshing the battery information.
    ///
    /// # Returns
    ///
    /// A `Battery` instance with the retrieved information.
    fn get_battery_info(
        index: Option<usize>,
        time_format: TimeFormat,
        temp_unit: TempUnit,
        refresh_interval: Duration,
    ) -> PyResult<Self> {
        let manager = Manager::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create manager: {}",
                e
            ))
        })?;
        let batteries: Vec<_> = manager
            .batteries()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to get batteries: {}",
                    e
                ))
            })?
            .collect();

        if batteries.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "No batteries found",
            ));
        }

        let battery_index = index.unwrap_or(0);
        if battery_index >= batteries.len() {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Battery index out of range",
            ));
        }

        let battery = batteries[battery_index].as_ref().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to get battery: {}",
                e
            ))
        })?;

        let vendor = battery.vendor().map(|v| v.trim().to_string());
        let model = battery.model().map(|m| m.trim().to_string());
        let serial_number = battery.serial_number().map(|s| s.trim().to_string());
        let technology = format!("{}", battery.technology());
        let percent_full = Measurement::new(
            battery.state_of_charge().get::<percent>(),
            "%".to_string(),
            1,
        );
        let state = battery.state();
        let capacity = Measurement::new(
            battery.state_of_health().get::<percent>(),
            "%".to_string(),
            1,
        );
        let temperature = match temp_unit {
            TempUnit::DegC => battery
                .temperature()
                .map(|t| Measurement::new(t.get::<degree_celsius>(), "°C".to_string(), 1)),
            TempUnit::DegF => battery
                .temperature()
                .map(|t| Measurement::new(t.get::<degree_fahrenheit>(), "°F".to_string(), 1)),
        };
        let cycle_count = battery.cycle_count();
        let energy = Measurement::new(battery.energy().get::<watt_hour>(), "Wh".to_string(), 1);
        let energy_full = Measurement::new(
            battery.energy_full().get::<watt_hour>(),
            "Wh".to_string(),
            1,
        );
        let energy_full_design = Measurement::new(
            battery.energy_full_design().get::<watt_hour>(),
            "Wh".to_string(),
            1,
        );
        let energy_rate = Measurement::new(battery.energy_rate().get::<watt>(), "W".to_string(), 1);
        let voltage = Measurement::new(battery.voltage().get::<volt>(), "V".to_string(), 1);

        let time_to_empty = match time_format {
            TimeFormat::Seconds => battery
                .time_to_empty()
                .map(|t| format!("{:.1} seconds", t.value)),
            TimeFormat::Minutes => battery
                .time_to_empty()
                .map(|t| format!("{:.1} minutes", t.value / 60.0)),
            TimeFormat::Human => battery
                .time_to_empty()
                .map(|t| Duration::from_secs_f32(t.value.trunc()).to_human_time_string()),
        };
        let time_to_full = match time_format {
            TimeFormat::Seconds => battery
                .time_to_full()
                .map(|t| format!("{:.1} seconds", t.value)),
            TimeFormat::Minutes => battery
                .time_to_full()
                .map(|t| format!("{:.1} minutes", t.value / 60.0)),
            TimeFormat::Human => battery
                .time_to_full()
                .map(|t| Duration::from_secs_f32(t.value.trunc()).to_human_time_string()),
        };

        Ok(Battery {
            vendor,
            model,
            serial_number,
            technology,
            percent_full,
            state,
            capacity,
            temperature,
            cycle_count,
            energy,
            energy_full,
            energy_full_design,
            energy_rate,
            voltage,
            time_to_empty,
            time_to_full,
            time_format,
            temp_unit,
            last_refresh: Instant::now(),
            refresh_interval,
            battery_index,
        })
    }

    fn refresh_if_needed(&mut self) -> PyResult<()> {
        if self.last_refresh.elapsed() >= self.refresh_interval {
            self.refresh(Some(self.battery_index))?;
            self.last_refresh = Instant::now();
        }
        Ok(())
    }
}

#[pymethods]
impl Battery {
    /// Creates a new `Battery` instance.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the battery to retrieve (optional).
    /// * `time_format` - The format for displaying time (default: `TimeFormat::Human`).
    /// * `temp_unit` - The unit for displaying temperature (default: `TempUnit::DegF`).
    /// * `refresh_interval` - The interval for refreshing the battery information (default: 500 ms).
    ///
    /// # Returns
    ///
    /// A `Battery` instance with the retrieved information.
    #[new]
    #[pyo3(signature = (index=None, time_format=TimeFormat::Human, temp_unit=TempUnit::DegF, refresh_interval=500))]
    fn new(
        index: Option<usize>,
        time_format: TimeFormat,
        temp_unit: TempUnit,
        refresh_interval: u64,
    ) -> PyResult<Self> {
        Battery::get_battery_info(
            index,
            time_format,
            temp_unit,
            Duration::from_millis(refresh_interval),
        )
    }

    /// Gets/sets the refresh interval.
    ///
    /// # Arguments
    ///
    /// * `interval_ms` - The refresh interval in milliseconds.
    #[setter]
    fn set_refresh_interval(&mut self, interval_ms: u64) -> PyResult<()> {
        self.refresh_interval = Duration::from_millis(interval_ms);
        Ok(())
    }

    /// Returns the refresh interval in milliseconds.
    #[getter]
    fn refresh_interval(&self) -> PyResult<u64> {
        Ok(self.refresh_interval.as_millis() as u64)
    }

    /// Returns the vendor of the battery.
    #[getter]
    fn vendor(&mut self) -> PyResult<Option<String>> {
        Ok(self.vendor.clone())
    }

    /// Returns the model of the battery.
    #[getter]
    fn model(&mut self) -> PyResult<Option<String>> {
        Ok(self.model.clone())
    }

    /// Returns the serial number of the battery.
    #[getter]
    fn serial_number(&mut self) -> PyResult<Option<String>> {
        Ok(self.serial_number.clone())
    }

    /// Returns the technology of the battery.
    #[getter]
    fn technology(&mut self) -> PyResult<String> {
        Ok(self.technology.clone())
    }

    /// Returns the percentage of the battery that is full.
    #[getter]
    fn percent(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.percent_full.clone())
    }

    /// Returns the state of the battery (charging, discharging, etc.).
    #[getter]
    fn state(&mut self) -> PyResult<String> {
        self.refresh_if_needed()?;
        Ok(format!("{:?}", self.state))
    }

    /// Returns the capacity of the battery.
    #[getter]
    fn capacity(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.capacity.clone())
    }

    /// Returns the temperature of the battery.
    #[getter]
    fn temperature(&mut self) -> PyResult<Option<Measurement>> {
        self.refresh_if_needed()?;
        Ok(self.temperature.clone())
    }

    /// Returns the cycle count of the battery.
    #[getter]
    fn cycle_count(&mut self) -> PyResult<Option<u32>> {
        self.refresh_if_needed()?;
        Ok(self.cycle_count)
    }

    /// Returns the current energy of the battery.
    #[getter]
    fn energy(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.energy.clone())
    }

    /// Returns the full energy of the battery.
    #[getter]
    fn energy_full(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.energy_full.clone())
    }

    /// Returns the design energy of the battery.
    #[getter]
    fn energy_full_design(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.energy_full_design.clone())
    }

    /// Returns the energy rate of the battery.
    #[getter]
    fn energy_rate(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.energy_rate.clone())
    }

    /// Returns the voltage of the battery.
    #[getter]
    fn voltage(&mut self) -> PyResult<Measurement> {
        self.refresh_if_needed()?;
        Ok(self.voltage.clone())
    }

    /// Returns the time to empty the battery.
    #[getter]
    fn time_to_empty(&mut self) -> PyResult<Option<String>> {
        self.refresh_if_needed()?;
        Ok(self.time_to_empty.clone())
    }

    /// Returns the time to fully charge the battery.
    #[getter]
    fn time_to_full(&mut self) -> PyResult<Option<String>> {
        self.refresh_if_needed()?;
        Ok(self.time_to_full.clone())
    }

    #[getter]
    fn hello(&mut self) -> PyResult<String> {
        self.refresh_if_needed()?;
        Ok("hello".to_string())
    }

    /// Refreshes the battery information.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the battery to retrieve (optional).
    ///
    /// # Returns
    ///
    /// An empty `PyResult` indicating success or failure.
    #[pyo3(signature = (index=None))]
    fn refresh(&mut self, index: Option<usize>) -> PyResult<()> {
        // println!("Refreshing battery information...");
        let battery = Battery::get_battery_info(
            index,
            self.time_format,
            self.temp_unit,
            self.refresh_interval,
        )?;
        // Only update the fields that could possibly change.
        self.percent_full = battery.percent_full;
        self.state = battery.state;
        self.capacity = battery.capacity;
        self.temperature = battery.temperature;
        self.cycle_count = battery.cycle_count;
        self.energy = battery.energy;
        self.energy_full = battery.energy_full;
        self.energy_full_design = battery.energy_full_design;
        self.energy_rate = battery.energy_rate;
        self.voltage = battery.voltage;
        self.time_to_empty = battery.time_to_empty;
        self.time_to_full = battery.time_to_full;
        Ok(())
    }

    /// Returns all battery information as a Python dictionary.
    fn as_dict(&self, py: Python) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);

        dict.set_item("vendor", self.vendor.clone())?;
        dict.set_item("model", self.model.clone())?;
        dict.set_item("serial_number", self.serial_number.clone())?;
        dict.set_item("technology", self.technology.clone())?;
        dict.set_item("percent", (self.percent_full.value, self.percent_full.units.clone()))?;
        dict.set_item("state", format!("{}", self.state))?;
        dict.set_item("capacity", (self.capacity.value, self.capacity.units.clone()))?;
        dict.set_item(
            "temperature",
            self.temperature
                .as_ref()
                .map(|t| (t.value, t.units.clone()))
                .unwrap_or_default(),
        )?;
        dict.set_item(
            "cycle_count",
            self.cycle_count.map(|c| c.to_string()).unwrap_or_default(),
        )?;
        dict.set_item("energy", (self.energy.value, self.energy.units.clone()))?;
        dict.set_item("energy_full", (self.energy_full.value, self.energy_full.units.clone()))?;
        dict.set_item(
            "energy_full_design",
            (self.energy_full_design.value, self.energy_full_design.units.clone()),
        )?;
        dict.set_item("energy_rate", (self.energy_rate.value, self.energy_rate.units.clone()))?;
        dict.set_item("voltage", (self.voltage.value, self.voltage.units.clone()))?;
        dict.set_item(
            "time_to_empty",
            self.time_to_empty.clone().unwrap_or_default(),
        )?;
        dict.set_item(
            "time_to_full",
            self.time_to_full.clone().unwrap_or_default(),
        )?;
        dict.set_item("battery_index", self.battery_index)?;

        Ok(dict.into())
    }
}
