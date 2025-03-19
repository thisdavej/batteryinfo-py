# batteryinfo

This project provides cross-platform Python bindings specifically designed to access system battery information. Using the Rust `battery` crate as its foundation, it allows you to retrieve battery status, percent full, capacity, and more, regardless of the operating system.

## Setup

### Python

Install the latest version with

```sh
pip install batteryinfo
```

## Usage

Here are some examples of how to use batteryinfo in Python:

### Importing the module

```python
import batteryinfo
```

### Creating a Battery object

```py
battery = batteryinfo.Battery()
```

Other options

```py
# Create an instance of Battery with specific time format and temperature unit
battery = batteryinfo.Battery(index=0, time_format=batteryinfo.TimeFormat.Human, temp_unit=batteryinfo.TempUnit.DegC)

# Create an instance of Battery with specific time format, temperature unit, and refresh interval
battery = batteryinfo.Battery(time_format=batteryinfo.TimeFormat.Human, temp_unit=batteryinfo.TempUnit.DegF, refresh_interval=600)
```

### Accessing Battery properties

```python
print(f"Vendor: {battery.vendor}")
print(f"Model: {battery.model}")
print(f"Serial Number: {battery.serial_number}")
print(f"Technology: {battery.technology}")
print(f"Percent Full: {battery.percent}")
print(f"State: {battery.state}")
print(f"Capacity: {battery.capacity}")
print(f"Temperature: {battery.temperature if battery.temperature else 'N/A'}")
print(f"Cycle Count: {battery.cycle_count}")
print(f"Energy: {battery.energy}")
print(f"Energy Full: {battery.energy_full}")
print(f"Energy Full Design: {battery.energy_full_design}")
print(f"Energy Rate: {battery.energy_rate}")
print(f"Voltage: {battery.voltage}")
print(f"Time to Empty: {battery.time_to_empty}")
print(f"Time to Full: {battery.time_to_full}")
```

## Available Properties

The following properties are available on the `Battery` object:

- `vendor`: The vendor of the battery (optional).
- `model`: The model of the battery (optional).
- `serial_number`: The serial number of the battery (optional).
- `technology`: The technology of the battery.
- `percent`: The percentage of the battery that is full (as a `Measurement` object).
- `state`: The state of the battery (Charging, Discharging, Full, Empty, Unknown).
- `capacity`: The capacity of the battery (as a `Measurement` object).
- `temperature`: The temperature of the battery (as a `Measurement` object).
- `cycle_count`: The cycle count of the battery.
- `energy`: The current energy of the battery (as a `Measurement` object).
- `energy_full`: The full energy of the battery (as a `Measurement` object).
- `energy_full_design`: The design energy of the battery (as a `Measurement` object).
- `energy_rate`: The energy rate of the battery (as a `Measurement` object).
- `voltage`: The voltage of the battery (as a `Measurement` object).
- `time_to_empty`: The time to empty the battery.
- `time_to_full`: The time to fully charge the battery.

## Battery Constructor Parameters

The `Battery` constructor accepts the following parameters:

- `index` (optional): The index of the battery to interact with. Default is `0`. An index of `1` is used if you have a second battery in your system.
- `time_format` (optional): The format to display time. Possible values are:
  - `TimeFormat.Seconds`: Display time in seconds.
  - `TimeFormat.Minutes`: Display time in minutes.
  - `TimeFormat.Human`: Display time in a human-readable format. For example, `1h,25m,52s`. (Default)
- `temp_unit` (optional): The unit to display temperature. Possible values are:
  - `TempUnit.DegC`: Display temperature in degrees Celsius.
  - `TempUnit.DegF`: Display temperature in degrees Fahrenheit. (Default)
- `refresh_interval` (optional): The interval in milliseconds to refresh battery information. Default is `500` milliseconds.

### Setting the Refresh Interval

The `refresh_interval` controls how frequently the battery data is updated, defaulting to 500 milliseconds. If needed, you can configure this parameter during object creation or later. It's only relevant for applications that repeatedly query battery information, as a single-use instance won't benefit from it.

Note: The 500ms refresh_interval (or value you have requested) acts as a cache timeout. When you request battery data like voltage, the system updates the values only if they're older than 500ms. If they're more recent, the cached values are returned.

Example usage:

```py
# Passing refresh_interval in the constructor
battery = batteryinfo.Battery(refresh_interval=1000)

# Setting refresh_interval after creating the Battery object
battery.refresh_interval = 1000
```

There is also an option to manually refresh the battery information, but the `refresh_interval` (and likely the default value of 500 ms) will accomplish the goal well in most situations.

```python
battery.refresh()
```

## Measurement Object

The `Measurement` object has the following properties and methods:

- `value`: The value of the measurement.
- `units`: The units of the measurement.

Example usage:

```py
# This first option provides the value and the units together.
print(f"Percent full: {battery.percent}")
# Provides the numeric value on its own so it can be used for comparisons and calculations.
print(f"Percent full raw value: {battery.percent.value}")
# Provides the units of measure such "%" or "V" for volts.
print(f"Percent full units: {battery.percent.units}")
```

Output

```text
Percent full: 88.2%
Percent full raw value: 88.2
Percent full units: %
```

## Enums

The following enums are available:

### TimeFormat

- `Seconds`: Display time in seconds.
- `Minutes`: Display time in minutes.
- `Human`: Display time in a human-readable format.

### TempUnit

- `DegC`: Display temperature in degrees Celsius.
- `DegF`: Display temperature in degrees Fahrenheit.

### Using the `as_dict` Method

The `as_dict` method returns all battery information as a Python dictionary. For fields represented by `Measurement` objects, the method returns a tuple `(value, units)`.

```python
# Get all battery information as a dictionary
battery_info = battery.as_dict()

# Print the dictionary
print(battery_info)

# Example output:
# {
#     "vendor": "BatteryVendor",
#     "model": "BatteryModel",
#     "serial_number": "123456789",
#     "technology": "Li-ion",
#     "percent": (71.1, "%"),
#     "state": "Charging",
#     "capacity": (95.0, "%"),
#     "temperature": (86.2, "°F"),
#     "cycle_count": 300,
#     "energy": (50.0, "Wh"),
#     "energy_full": (60.0, "Wh"),
#     "energy_full_design": (65.0, "Wh"),
#     "energy_rate": (10.0, "W"),
#     "voltage": (12.5, "V"),
#     "time_to_empty": None,
#     "time_to_full": "1h,5m,19s",
#     "battery_index": 0
# }

# Access specific fields
print("Vendor:", battery_info.get("vendor"))
print("Percent:", battery_info.get("percent"))  # Example: (71.1, "%")
print("Energy:", battery_info.get("energy"))    # Example: (50.0, "Wh")
```

### Python Example - Displaying Battery Information Based on State

```python
battery = batteryinfo.Battery()

state = battery.state
percent = battery.percent.value
if state == "Charging":
    time_to_full = battery.time_to_full
    print(f"Battery: {percent} (⇡ charging - full in {time_to_full})")
elif state == "Discharging":
    time_to_empty = battery.time_to_empty
    print(f"Battery: {percent} (⇣ discharging - empty in {time_to_empty})")
elif state == "Full":
    print(f"Battery: {percent} (✓ full)")
else:
    print(f"Battery: {percent} (state: {state})")
```

Example output:

```text
Battery: 70.4% (⇣ discharging - empty in 2h,40m,38s)
```

## License

This project is licensed under the MIT License.
