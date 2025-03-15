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

    def __init__(self, index: int = 0, time_format: str = "Human", temp_unit: str = "DegF", refresh_interval: int = 500) -> None: ...

    def refresh(self) -> None: ...

class TimeFormat:
    Seconds: str
    Minutes: str
    Human: str

class TempUnit:
    DegC: str
    DegF: str
