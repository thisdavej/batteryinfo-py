from typing import Optional, Union

class Measurement:
    value: float
    units: str

class Battery:
    vendor: Optional[str]
    model: Optional[str]
    serial_number: Optional[str]
    technology: str
    percent: Measurement
    state: str
    capacity: Measurement
    temperature: Optional[Measurement]
    cycle_count: Optional[int]
    energy: Measurement
    energy_full: Measurement
    energy_full_design: Measurement
    energy_rate: Measurement
    voltage: Measurement
    time_to_empty: Optional[str]
    time_to_full: Optional[str]

    @property
    def refresh_interval(self) -> int: ...
    @refresh_interval.setter
    def refresh_interval(self, value: int) -> None: ...
    def __init__(
        self,
        index: int = 0,
        time_format: str = "Human",
        temp_unit: str = "DegF",
        refresh_interval: int = 500,
    ) -> None: ...
    def refresh(self) -> None: ...
    def as_dict(self) -> dict[str, object]:
        """
        Returns all battery information as a dictionary.

        Measurement fields are represented as tuples of (value, units).
        Example:
            {
                "vendor": Optional[str],
                "model": Optional[str],
                "serial_number": Optional[str],
                "technology": str,
                "percent": tuple[float, str],
                "state": str,
                "capacity": tuple[float, str],
                "temperature": Optional[tuple[float, str]],
                "cycle_count": Optional[int],
                "energy": tuple[float, str],
                "energy_full": tuple[float, str],
                "energy_full_design": tuple[float, str],
                "energy_rate": tuple[float, str],
                "voltage": tuple[float, str],
                "time_to_empty": Optional[str],
                "time_to_full": Optional[str],
                "battery_index": int,
            }
        """
        ...

class TimeFormat:
    Seconds: str
    Minutes: str
    Human: str

class TempUnit:
    DegC: str
    DegF: str
