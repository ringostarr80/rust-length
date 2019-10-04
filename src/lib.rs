#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use self::regex::Regex;

use AstronomicUnit::*;
use ImperialUnit::*;
use MetricUnit::*;

#[derive(Clone)]
pub struct Length {
    pub unit: Unit,
    pub value: f64,
    original_string: String,
}

impl Length {
    const YARD_TO_METER_FACTOR: f64 = 0.9144;
    const LIGHTYEAR_TO_METER_FACTOR: f64 = 9_460_730_472_580_800.0;
    const LIGHTDAY_TO_LIGHTYEAR_FACTOR: f64 = 1.0 / 365.25;
    const LIGHTHOUR_TO_LIGHTYEAR_FACTOR: f64 = 1.0 / (365.25 * 24.0);
    const LIGHTMINUTE_TO_LIGHTYEAR_FACTOR: f64 = 1.0 / (365.25 * 24.0 * 60.0);
    const LIGHTSECOND_TO_LIGHTYEAR_FACTOR: f64 = 1.0 / (365.25 * 24.0 * 60.0 * 60.0);
    const ASTRONOMICAL_UNIT_TO_LIGHTYEAR_FACTOR: f64 = 149_597_870_700.0 / 9_460_730_472_580_800.0;
    const PARSEC_TO_ASTRONOMICAL_UNITS_FACTOR: f64 = 648_000.0 / PI;
    const PARSEC_TO_LIGHTYEAR_FACTOR: f64 =
        Length::ASTRONOMICAL_UNIT_TO_LIGHTYEAR_FACTOR * Length::PARSEC_TO_ASTRONOMICAL_UNITS_FACTOR;
    const KILOPARSEC_TO_LIGHTYEAR_FACTOR: f64 = Length::PARSEC_TO_LIGHTYEAR_FACTOR * 1_000.0;
    const MEGAPARSEC_TO_LIGHTYEAR_FACTOR: f64 = Length::PARSEC_TO_LIGHTYEAR_FACTOR * 1_000_000.0;

    /// Gets a new Length struct, that represents 0 meters.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let length = Length::new();
    ///
    /// assert_eq!(0.0, length.value);
    /// assert_eq!(Unit::Metric(Meter), length.unit);
    /// ```
    pub fn new() -> Length {
        Length {
            unit: Unit::Metric(Meter),
            value: 0.0,
            original_string: String::new(),
        }
    }

    /// Gets a new Length struct with the given value and unit.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let length = Length::new_value_unit(1.25, Unit::Metric(Kilometer));
    ///
    /// assert_eq!(1.25, length.value);
    /// assert_eq!(Unit::Metric(Kilometer), length.unit);
    /// ```
    pub fn new_value_unit<T: Into<f64>>(value: T, unit: Unit) -> Length {
        Length {
            unit: unit,
            value: value.into(),
            original_string: String::new(),
        }
    }

    /// Gets a new Option<Length>, that represents a length by a string.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let two_meters = Length::new_string("2m").unwrap();
    ///
    /// assert_eq!(2.0, two_meters.value);
    /// assert_eq!(Unit::Metric(Meter), two_meters.unit);
    /// ```
    pub fn new_string<S: Into<String>>(string: S) -> Option<Length> {
        lazy_static! {
            static ref RE_LENGTH: Regex =
                Regex::new(r"^\s*([0-9]+(\.[0-9]+)?)\s*([a-zA-Z]{1,3})\s*$").unwrap();
        }

        let real_string: String = string.into();

        let caps = RE_LENGTH.captures(real_string.as_str());
        if caps.is_none() {
            return None;
        }

        let cap = caps.unwrap();
        let original_string = String::from(&cap[0]);
        let value: f64 = match String::from(&cap[1]).parse() {
            Ok(val) => val,
            Err(_) => return None,
        };

        let unit = match &cap[3].parse::<Unit>() {
            Ok(parsed) => *parsed,
            Err(_) => return None,
        };

        Some(Length {
            unit: unit,
            value: value.into(),
            original_string: original_string,
        })
    }

    /// Gets the original string of the length, if it was called with new_string(...)
    ///
    /// # Example
    /// ```
    /// use length::Length;
    ///
    /// let two_meter = Length::new_string("2m").unwrap();
    /// let five_kilometer = Length::new_string("5 km").unwrap();
    ///
    /// assert_eq!("2m", two_meter.get_original_string());
    /// assert_eq!("5 km", five_kilometer.get_original_string());
    /// ```
    pub fn get_original_string(&self) -> String {
        self.original_string.clone()
    }

    /// Converts this length into the given unit and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let fivethousand_meter = five_kilometer.to(Unit::Metric(Meter));
    ///
    /// assert_eq!(5000.0, fivethousand_meter.value);
    /// assert_eq!(Unit::Metric(Meter), fivethousand_meter.unit);
    /// ```
    pub fn to(&self, destination_unit: Unit) -> Length {
        let mut self_cloned = self.clone();

        if self_cloned.unit == destination_unit {
            return self_cloned;
        }

        if self_cloned.unit.system() != destination_unit.system() {
            match destination_unit.system() {
                UnitSystem::Astronomic => match self_cloned.unit.system() {
                    UnitSystem::Metric => {
                        let source_in_m = self_cloned.to(Unit::Metric(Meter));
                        let ly = source_in_m.value / Length::LIGHTYEAR_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(ly, Unit::Astronomic(Lightyear));
                    }
                    UnitSystem::Imperial => {
                        let source_in_m = self_cloned.to(Unit::Metric(Meter));
                        let ly = source_in_m.value / Length::LIGHTYEAR_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(ly, Unit::Astronomic(Lightyear));
                    }
                    _ => {}
                },
                UnitSystem::Imperial => match self_cloned.unit.system() {
                    UnitSystem::Astronomic => {
                        let source_in_ly = self_cloned.to(Unit::Astronomic(Lightyear));
                        let m = source_in_ly.value * Length::LIGHTYEAR_TO_METER_FACTOR;
                        let yards = m / Length::YARD_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(yards, Unit::Imperial(Yard));
                    }
                    UnitSystem::Metric => {
                        let source_in_m = self_cloned.to(Unit::Metric(Meter));
                        let yards = source_in_m.value / Length::YARD_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(yards, Unit::Imperial(Yard));
                    }
                    _ => {}
                },
                UnitSystem::Metric => match self_cloned.unit.system() {
                    UnitSystem::Astronomic => {
                        let source_in_ly = self_cloned.to(Unit::Astronomic(Lightyear));
                        let m = source_in_ly.value * Length::LIGHTYEAR_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(m, Unit::Metric(Meter));
                    }
                    UnitSystem::Imperial => {
                        let source_in_yd = self_cloned.to(Unit::Imperial(Yard));
                        let meter = source_in_yd.value * Length::YARD_TO_METER_FACTOR;
                        self_cloned = Length::new_value_unit(meter, Unit::Metric(Meter));
                    }
                    _ => {}
                },
            }
        }

        let factor = self_cloned.unit.factor() * (1.0 / destination_unit.factor());
        Length::new_value_unit(self_cloned.value * factor, destination_unit)
    }
}

impl Default for Length {
    fn default() -> Length {
        Length::new()
    }
}

impl ToString for Length {
    fn to_string(&self) -> String {
        format!("{} {}", self.value, self.unit.to_string())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Unit {
    Astronomic(AstronomicUnit),
    Imperial(ImperialUnit),
    Metric(MetricUnit),
}

impl Unit {
    pub fn factor(&self) -> f64 {
        match self {
            Unit::Astronomic(system) => system.factor(),
            Unit::Metric(system) => system.factor(),
            Unit::Imperial(system) => system.factor(),
        }
    }

    pub fn is_astronomic(&self) -> bool {
        match self {
            Unit::Astronomic(_) => true,
            _ => false,
        }
    }

    pub fn is_imperial(&self) -> bool {
        match self {
            Unit::Imperial(_) => true,
            _ => false,
        }
    }

    pub fn is_metric(&self) -> bool {
        match self {
            Unit::Metric(_) => true,
            _ => false,
        }
    }

    pub fn system(&self) -> UnitSystem {
        match self {
            Unit::Astronomic(_) => UnitSystem::Astronomic,
            Unit::Imperial(_) => UnitSystem::Imperial,
            Unit::Metric(_) => UnitSystem::Metric,
        }
    }
}

impl FromStr for Unit {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "au" => Ok(Unit::Astronomic(AstronomicalUnit)),
            "ls" => Ok(Unit::Astronomic(Lightsecond)),
            "lm" => Ok(Unit::Astronomic(Lightminute)),
            "lh" => Ok(Unit::Astronomic(Lighthour)),
            "ld" => Ok(Unit::Astronomic(Lightday)),
            "ly" => Ok(Unit::Astronomic(Lightyear)),
            "pc" => Ok(Unit::Astronomic(Parsec)),
            "kpc" => Ok(Unit::Astronomic(Kiloparsec)),
            "Mpc" => Ok(Unit::Astronomic(Megaparsec)),
            "in" => Ok(Unit::Imperial(Inch)),
            "ft" => Ok(Unit::Imperial(Foot)),
            "yd" => Ok(Unit::Imperial(Yard)),
            "mi" => Ok(Unit::Imperial(Mile)),
            "ym" => Ok(Unit::Metric(Yoctometer)),
            "zm" => Ok(Unit::Metric(Zeptometer)),
            "am" => Ok(Unit::Metric(Attometer)),
            "fm" => Ok(Unit::Metric(Femtometer)),
            "pm" => Ok(Unit::Metric(Picometer)),
            "nm" => Ok(Unit::Metric(Nanometer)),
            "µm" => Ok(Unit::Metric(Micrometer)),
            "mm" => Ok(Unit::Metric(Millimeter)),
            "cm" => Ok(Unit::Metric(Centimeter)),
            "dm" => Ok(Unit::Metric(Decimeter)),
            "m" => Ok(Unit::Metric(Meter)),
            "dam" => Ok(Unit::Metric(Decameter)),
            "hm" => Ok(Unit::Metric(Hectometer)),
            "km" => Ok(Unit::Metric(Kilometer)),
            "Mm" => Ok(Unit::Metric(Megameter)),
            "Gm" => Ok(Unit::Metric(Gigameter)),
            "Tm" => Ok(Unit::Metric(Terameter)),
            "Pm" => Ok(Unit::Metric(Petameter)),
            "Em" => Ok(Unit::Metric(Exameter)),
            "Zm" => Ok(Unit::Metric(Zettameter)),
            "Ym" => Ok(Unit::Metric(Yottameter)),
            _ => Err("unable to parse string to Color-struct."),
        }
    }
}

#[derive(PartialEq)]
pub enum UnitSystem {
    Astronomic,
    Imperial,
    Metric,
}

trait UnitFactor {
    fn factor(&self) -> f64;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AstronomicUnit {
    AstronomicalUnit,
    Lightsecond,
    Lightminute,
    Lighthour,
    Lightday,
    Lightyear,
    Parsec,
    Kiloparsec,
    Megaparsec,
}

impl UnitFactor for AstronomicUnit {
    fn factor(&self) -> f64 {
        match self {
            AstronomicUnit::AstronomicalUnit => Length::ASTRONOMICAL_UNIT_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Lightsecond => Length::LIGHTSECOND_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Lightminute => Length::LIGHTMINUTE_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Lighthour => Length::LIGHTHOUR_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Lightday => Length::LIGHTDAY_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Lightyear => 1.0,
            AstronomicUnit::Parsec => Length::PARSEC_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Kiloparsec => Length::KILOPARSEC_TO_LIGHTYEAR_FACTOR,
            AstronomicUnit::Megaparsec => Length::MEGAPARSEC_TO_LIGHTYEAR_FACTOR,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImperialUnit {
    Inch,
    Foot,
    Yard,
    Mile,
}

impl UnitFactor for ImperialUnit {
    fn factor(&self) -> f64 {
        match self {
            ImperialUnit::Inch => 1.0,
            ImperialUnit::Foot => 12.0,
            ImperialUnit::Yard => 36.0,
            ImperialUnit::Mile => 63360.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MetricUnit {
    Yoctometer,
    Zeptometer,
    Attometer,
    Femtometer,
    Picometer,
    Nanometer,
    Micrometer,
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
    Decameter,
    Hectometer,
    Kilometer,
    Megameter,
    Gigameter,
    Terameter,
    Petameter,
    Exameter,
    Zettameter,
    Yottameter,
}

impl UnitFactor for MetricUnit {
    fn factor(&self) -> f64 {
        match self {
            MetricUnit::Yoctometer => 0.000_000_000_000_000_000_000_001,
            MetricUnit::Zeptometer => 0.000_000_000_000_000_000_001,
            MetricUnit::Attometer => 0.000_000_000_000_000_001,
            MetricUnit::Femtometer => 0.000_000_000_000_001,
            MetricUnit::Picometer => 0.000_000_000_001,
            MetricUnit::Nanometer => 0.000_000_001,
            MetricUnit::Micrometer => 0.000_001,
            MetricUnit::Millimeter => 0.001,
            MetricUnit::Centimeter => 0.01,
            MetricUnit::Decimeter => 0.1,
            MetricUnit::Meter => 1.0,
            MetricUnit::Decameter => 10.0,
            MetricUnit::Hectometer => 100.0,
            MetricUnit::Kilometer => 1_000.0,
            MetricUnit::Megameter => 1_000_000.0,
            MetricUnit::Gigameter => 1_000_000_000.0,
            MetricUnit::Terameter => 1_000_000_000_000.0,
            MetricUnit::Petameter => 1_000_000_000_000_000.0,
            MetricUnit::Exameter => 1_000_000_000_000_000_000.0,
            MetricUnit::Zettameter => 1_000_000_000_000_000_000_000.0,
            MetricUnit::Yottameter => 1_000_000_000_000_000_000_000_000.0,
        }
    }
}

impl Hash for Unit {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Astronomic(AstronomicalUnit) => String::from("au"),
            Unit::Astronomic(Lightsecond) => String::from("ls"),
            Unit::Astronomic(Lightminute) => String::from("lm"),
            Unit::Astronomic(Lighthour) => String::from("lh"),
            Unit::Astronomic(Lightday) => String::from("ld"),
            Unit::Astronomic(Lightyear) => String::from("ly"),
            Unit::Astronomic(Parsec) => String::from("pc"),
            Unit::Astronomic(Kiloparsec) => String::from("kpc"),
            Unit::Astronomic(Megaparsec) => String::from("Mpc"),
            Unit::Imperial(Inch) => String::from("in"),
            Unit::Imperial(Foot) => String::from("ft"),
            Unit::Imperial(Yard) => String::from("yd"),
            Unit::Imperial(Mile) => String::from("mi"),
            Unit::Metric(Yoctometer) => String::from("ym"),
            Unit::Metric(Zeptometer) => String::from("zm"),
            Unit::Metric(Attometer) => String::from("am"),
            Unit::Metric(Femtometer) => String::from("fm"),
            Unit::Metric(Picometer) => String::from("pm"),
            Unit::Metric(Nanometer) => String::from("nm"),
            Unit::Metric(Micrometer) => String::from("µm"),
            Unit::Metric(Millimeter) => String::from("mm"),
            Unit::Metric(Centimeter) => String::from("cm"),
            Unit::Metric(Decimeter) => String::from("dm"),
            Unit::Metric(Meter) => String::from("m"),
            Unit::Metric(Decameter) => String::from("dam"),
            Unit::Metric(Hectometer) => String::from("hm"),
            Unit::Metric(Kilometer) => String::from("km"),
            Unit::Metric(Megameter) => String::from("Mm"),
            Unit::Metric(Gigameter) => String::from("Gm"),
            Unit::Metric(Terameter) => String::from("Tm"),
            Unit::Metric(Petameter) => String::from("Pm"),
            Unit::Metric(Exameter) => String::from("Em"),
            Unit::Metric(Zettameter) => String::from("Zm"),
            Unit::Metric(Yottameter) => String::from("Ym"),
        }
    }
}
