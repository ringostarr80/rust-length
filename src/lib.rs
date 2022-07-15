#[macro_use]
extern crate lazy_static;

use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use regex::Regex;

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
    pub fn new() -> Self {
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
    /// let length1 = Length::new_value_unit(1.25, Unit::Metric(Kilometer));
    /// let length2 = Length::new_value_unit(1.25, Kilometer);
    ///
    /// assert_eq!(1.25, length1.value);
    /// assert_eq!(Unit::Metric(Kilometer), length1.unit);
    /// assert_eq!(1.25, length2.value);
    /// assert_eq!(Unit::Metric(Kilometer), length2.unit);
    /// ```
    pub fn new_value_unit<T: Into<f64>, U: Into<Unit>>(value: T, unit: U) -> Self {
        Length {
            unit: unit.into(),
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
    pub fn new_string<S: Into<String>>(string: S) -> Option<Self> {
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

    /// Gets a normalized Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let fivethousand_meter = Length::new_string("5000m").unwrap();
    /// let five_kilometer = fivethousand_meter.normalize();
    ///
    /// assert_eq!(5.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn normalize(&self) -> Self {
        let mut normalized_length = self.clone();

        let mut done = false;
        let mut iterations = 0;
        while !done && iterations < 10 {
            iterations += 1;
            if normalized_length.value < 1.0 {
                let smaller_unit = normalized_length.unit.smaller_unit();
                done = match smaller_unit {
                    Some(unit) => {
                        normalized_length.to_by_ref(unit);
                        false
                    }
                    None => true,
                };
            } else {
                let greater_unit = normalized_length.unit.greater_unit();
                done = match greater_unit {
                    Some(unit) => {
                        let test_normalized = normalized_length.to(unit);
                        if test_normalized.value >= 1.0 {
                            normalized_length = test_normalized;
                            false
                        } else {
                            true
                        }
                    }
                    None => true,
                };
            }
        }

        normalized_length
    }

    /// Gets a normalized Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut fivethousand_meter = Length::new_string("5000m").unwrap();
    /// fivethousand_meter.normalize_by_ref();
    ///
    /// assert_eq!(5.0, fivethousand_meter.value);
    /// assert_eq!(Unit::Metric(Kilometer), fivethousand_meter.unit);
    /// ```
    pub fn normalize_by_ref(&mut self) -> &mut Self {
        let normalized = self.normalize();
        self.value = normalized.value;
        self.unit = normalized.unit;

        self
    }

    /// Converts this length into the given unit and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let fivethousand_meter1 = five_kilometer.to(Unit::Metric(Meter));
    /// let fivethousand_meter2 = five_kilometer.to(Meter);
    ///
    /// assert_eq!(5000.0, fivethousand_meter1.value);
    /// assert_eq!(Unit::Metric(Meter), fivethousand_meter1.unit);
    /// assert_eq!(5000.0, fivethousand_meter2.value);
    /// assert_eq!(Unit::Metric(Meter), fivethousand_meter2.unit);
    /// ```
    pub fn to<T: Into<Unit>>(&self, destination_unit: T) -> Self {
        let destination_unit = destination_unit.into();

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

    /// Converts this length into the given unit.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut five_kilometer = Length::new_string("5km").unwrap();
    /// five_kilometer.to_by_ref(Unit::Metric(Meter));
    ///
    /// assert_eq!(5000.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Meter), five_kilometer.unit);
    ///
    /// five_kilometer.to_by_ref(Kilometer);
    ///
    /// assert_eq!(5.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn to_by_ref<T: Into<Unit>>(&mut self, destination_unit: T) -> &mut Self {
        let new_length = self.to(destination_unit);
        self.value = new_length.value;
        self.unit = new_length.unit;

        self
    }

    /// Adds the length and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let twothousand_meter = Length::new_string("2000m").unwrap();
    /// let seven_kilometer = five_kilometer.add(twothousand_meter);
    ///
    /// assert_eq!(7.0, seven_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), seven_kilometer.unit);
    /// ```
    pub fn add(&self, length: Length) -> Self {
        let length_with_source_unit = length.to(self.unit);
        Length {
            value: self.value + length_with_source_unit.value,
            unit: self.unit,
            ..Default::default()
        }
    }

    /// Adds the length.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut five_kilometer = Length::new_string("5km").unwrap();
    /// let twothousand_meter = Length::new_string("2000m").unwrap();
    /// five_kilometer.add_by_ref(twothousand_meter);
    ///
    /// assert_eq!(7.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn add_by_ref(&mut self, length: Length) -> &mut Self {
        let length_with_source_unit = length.to(self.unit);
        self.value += length_with_source_unit.value;
        self
    }

    /// Subtracts the length and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let twothousand_meter = Length::new_string("2000m").unwrap();
    /// let three_kilometer = five_kilometer.subtract(twothousand_meter);
    ///
    /// assert_eq!(3.0, three_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), three_kilometer.unit);
    /// ```
    pub fn subtract(&self, length: Length) -> Self {
        let length_with_source_unit = length.to(self.unit);
        Length {
            value: self.value - length_with_source_unit.value,
            unit: self.unit,
            ..Default::default()
        }
    }

    /// Subtracts the length.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut five_kilometer = Length::new_string("5km").unwrap();
    /// let twothousand_meter = Length::new_string("2000m").unwrap();
    /// five_kilometer.subtract_by_ref(twothousand_meter);
    ///
    /// assert_eq!(3.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn subtract_by_ref(&mut self, length: Length) -> &mut Self {
        let length_with_source_unit = length.to(self.unit);
        self.value -= length_with_source_unit.value;
        self
    }

    /// Multiplies the length and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let fifty_kilometer = five_kilometer.multiply_by(10);
    ///
    /// assert_eq!(50.0, fifty_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), fifty_kilometer.unit);
    /// ```
    pub fn multiply_by<T: Into<f64>>(&self, factor: T) -> Self {
        let real_factor: f64 = factor.into();
        Length {
            value: self.value * real_factor,
            unit: self.unit,
            ..Default::default()
        }
    }

    /// Multiplies the length by a factor.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut five_kilometer = Length::new_string("5km").unwrap();
    /// five_kilometer.multiply_by_ref(10);
    ///
    /// assert_eq!(50.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn multiply_by_ref<T: Into<f64>>(&mut self, factor: T) -> &mut Self {
        let real_factor: f64 = factor.into();
        self.value *= real_factor;
        self
    }

    /// Divides the length and returns a new Length-struct.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let five_kilometer = Length::new_string("5km").unwrap();
    /// let one_kilometer = five_kilometer.divide_by(5);
    ///
    /// assert_eq!(1.0, one_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), one_kilometer.unit);
    /// ```
    pub fn divide_by<T: Into<f64>>(&self, factor: T) -> Self {
        let real_factor: f64 = factor.into();
        Length {
            value: self.value / real_factor,
            unit: self.unit,
            ..Default::default()
        }
    }

    /// Divides the length by a factor.
    ///
    /// # Example
    /// ```
    /// use length::{Length, Unit, MetricUnit::*};
    ///
    /// let mut five_kilometer = Length::new_string("5km").unwrap();
    /// five_kilometer.divide_by_ref(5);
    ///
    /// assert_eq!(1.0, five_kilometer.value);
    /// assert_eq!(Unit::Metric(Kilometer), five_kilometer.unit);
    /// ```
    pub fn divide_by_ref<T: Into<f64>>(&mut self, factor: T) -> &mut Self {
        let real_factor: f64 = factor.into();
        self.value /= real_factor;
        self
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
    /// This method is mainly intended for internal use only.
    pub fn factor(&self) -> f64 {
        match self {
            Unit::Astronomic(system) => system.factor(),
            Unit::Metric(system) => system.factor(),
            Unit::Imperial(system) => system.factor(),
        }
    }

    /// This method is mainly intended for internal use only.
    pub fn is_astronomic(&self) -> bool {
        match self {
            Unit::Astronomic(_) => true,
            _ => false,
        }
    }

    /// This method is mainly intended for internal use only.
    pub fn is_imperial(&self) -> bool {
        match self {
            Unit::Imperial(_) => true,
            _ => false,
        }
    }

    /// This method is mainly intended for internal use only.
    pub fn is_metric(&self) -> bool {
        match self {
            Unit::Metric(_) => true,
            _ => false,
        }
    }

    /// This method is mainly intended for internal use only.
    pub fn system(&self) -> UnitSystem {
        match self {
            Unit::Astronomic(_) => UnitSystem::Astronomic,
            Unit::Imperial(_) => UnitSystem::Imperial,
            Unit::Metric(_) => UnitSystem::Metric,
        }
    }
}

impl SiblingUnit for Unit {
    fn smaller_unit(&self) -> Option<Unit> {
        match self {
            Unit::Astronomic(astronomic_unit) => astronomic_unit.smaller_unit(),
            Unit::Imperial(imperial_unit) => imperial_unit.smaller_unit(),
            Unit::Metric(metric_unit) => metric_unit.smaller_unit(),
        }
    }

    fn greater_unit(&self) -> Option<Unit> {
        match self {
            Unit::Astronomic(astronomic_unit) => astronomic_unit.greater_unit(),
            Unit::Imperial(imperial_unit) => imperial_unit.greater_unit(),
            Unit::Metric(metric_unit) => metric_unit.greater_unit(),
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
            _ => Err("unable to parse string to Unit-enum."),
        }
    }
}

impl From<AstronomicUnit> for Unit {
    fn from(item: AstronomicUnit) -> Self {
        match item {
            AstronomicUnit::AstronomicalUnit => Unit::Astronomic(AstronomicUnit::AstronomicalUnit),
            AstronomicUnit::Lightsecond => Unit::Astronomic(AstronomicUnit::Lightsecond),
            AstronomicUnit::Lightminute => Unit::Astronomic(AstronomicUnit::Lightminute),
            AstronomicUnit::Lighthour => Unit::Astronomic(AstronomicUnit::Lighthour),
            AstronomicUnit::Lightday => Unit::Astronomic(AstronomicUnit::Lightday),
            AstronomicUnit::Lightyear => Unit::Astronomic(AstronomicUnit::Lightyear),
            AstronomicUnit::Parsec => Unit::Astronomic(AstronomicUnit::Parsec),
            AstronomicUnit::Kiloparsec => Unit::Astronomic(AstronomicUnit::Kiloparsec),
            AstronomicUnit::Megaparsec => Unit::Astronomic(AstronomicUnit::Megaparsec),
        }
    }
}

impl From<ImperialUnit> for Unit {
    fn from(item: ImperialUnit) -> Self {
        match item {
            ImperialUnit::Inch => Unit::Imperial(ImperialUnit::Inch),
            ImperialUnit::Foot => Unit::Imperial(ImperialUnit::Foot),
            ImperialUnit::Yard => Unit::Imperial(ImperialUnit::Yard),
            ImperialUnit::Mile => Unit::Imperial(ImperialUnit::Mile),
        }
    }
}

impl From<MetricUnit> for Unit {
    fn from(item: MetricUnit) -> Self {
        match item {
            MetricUnit::Yoctometer => Unit::Metric(MetricUnit::Yoctometer),
            MetricUnit::Zeptometer => Unit::Metric(MetricUnit::Zeptometer),
            MetricUnit::Attometer => Unit::Metric(MetricUnit::Attometer),
            MetricUnit::Femtometer => Unit::Metric(MetricUnit::Femtometer),
            MetricUnit::Picometer => Unit::Metric(MetricUnit::Picometer),
            MetricUnit::Nanometer => Unit::Metric(MetricUnit::Nanometer),
            MetricUnit::Micrometer => Unit::Metric(MetricUnit::Micrometer),
            MetricUnit::Millimeter => Unit::Metric(MetricUnit::Millimeter),
            MetricUnit::Centimeter => Unit::Metric(MetricUnit::Centimeter),
            MetricUnit::Decimeter => Unit::Metric(MetricUnit::Decimeter),
            MetricUnit::Meter => Unit::Metric(MetricUnit::Meter),
            MetricUnit::Decameter => Unit::Metric(MetricUnit::Decameter),
            MetricUnit::Hectometer => Unit::Metric(MetricUnit::Hectometer),
            MetricUnit::Kilometer => Unit::Metric(MetricUnit::Kilometer),
            MetricUnit::Megameter => Unit::Metric(MetricUnit::Megameter),
            MetricUnit::Gigameter => Unit::Metric(MetricUnit::Gigameter),
            MetricUnit::Terameter => Unit::Metric(MetricUnit::Terameter),
            MetricUnit::Petameter => Unit::Metric(MetricUnit::Petameter),
            MetricUnit::Exameter => Unit::Metric(MetricUnit::Exameter),
            MetricUnit::Zettameter => Unit::Metric(MetricUnit::Zettameter),
            MetricUnit::Yottameter => Unit::Metric(MetricUnit::Yottameter),
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

trait SiblingUnit {
    fn smaller_unit(&self) -> Option<Unit>;
    fn greater_unit(&self) -> Option<Unit>;
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

impl SiblingUnit for AstronomicUnit {
    fn smaller_unit(&self) -> Option<Unit> {
        match self {
            AstronomicUnit::AstronomicalUnit => None,
            AstronomicUnit::Lightsecond => Some(Unit::Astronomic(AstronomicUnit::AstronomicalUnit)),
            AstronomicUnit::Lightminute => Some(Unit::Astronomic(AstronomicUnit::Lightsecond)),
            AstronomicUnit::Lighthour => Some(Unit::Astronomic(AstronomicUnit::Lightminute)),
            AstronomicUnit::Lightday => Some(Unit::Astronomic(AstronomicUnit::Lighthour)),
            AstronomicUnit::Lightyear => Some(Unit::Astronomic(AstronomicUnit::Lightday)),
            AstronomicUnit::Parsec => Some(Unit::Astronomic(AstronomicUnit::Lightyear)),
            AstronomicUnit::Kiloparsec => Some(Unit::Astronomic(AstronomicUnit::Parsec)),
            AstronomicUnit::Megaparsec => Some(Unit::Astronomic(AstronomicUnit::Kiloparsec)),
        }
    }

    fn greater_unit(&self) -> Option<Unit> {
        match self {
            AstronomicUnit::AstronomicalUnit => Some(Unit::Astronomic(AstronomicUnit::Lightsecond)),
            AstronomicUnit::Lightsecond => Some(Unit::Astronomic(AstronomicUnit::Lightminute)),
            AstronomicUnit::Lightminute => Some(Unit::Astronomic(AstronomicUnit::Lighthour)),
            AstronomicUnit::Lighthour => Some(Unit::Astronomic(AstronomicUnit::Lightday)),
            AstronomicUnit::Lightday => Some(Unit::Astronomic(AstronomicUnit::Lightyear)),
            AstronomicUnit::Lightyear => Some(Unit::Astronomic(AstronomicUnit::Parsec)),
            AstronomicUnit::Parsec => Some(Unit::Astronomic(AstronomicUnit::Kiloparsec)),
            AstronomicUnit::Kiloparsec => Some(Unit::Astronomic(AstronomicUnit::Megaparsec)),
            AstronomicUnit::Megaparsec => None,
        }
    }
}

impl ToString for AstronomicUnit {
    fn to_string(&self) -> String {
        match self {
            AstronomicalUnit => String::from("au"),
            Lightsecond => String::from("ls"),
            Lightminute => String::from("lm"),
            Lighthour => String::from("lh"),
            Lightday => String::from("ld"),
            Lightyear => String::from("ly"),
            Parsec => String::from("pc"),
            Kiloparsec => String::from("kpc"),
            Megaparsec => String::from("Mpc"),
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

impl SiblingUnit for ImperialUnit {
    fn smaller_unit(&self) -> Option<Unit> {
        match self {
            ImperialUnit::Inch => None,
            ImperialUnit::Foot => Some(Unit::Imperial(ImperialUnit::Inch)),
            ImperialUnit::Yard => Some(Unit::Imperial(ImperialUnit::Foot)),
            ImperialUnit::Mile => Some(Unit::Imperial(ImperialUnit::Yard)),
        }
    }

    fn greater_unit(&self) -> Option<Unit> {
        match self {
            ImperialUnit::Inch => Some(Unit::Imperial(ImperialUnit::Foot)),
            ImperialUnit::Foot => Some(Unit::Imperial(ImperialUnit::Yard)),
            ImperialUnit::Yard => Some(Unit::Imperial(ImperialUnit::Mile)),
            ImperialUnit::Mile => None,
        }
    }
}

impl ToString for ImperialUnit {
    fn to_string(&self) -> String {
        match self {
            Inch => String::from("in"),
            Foot => String::from("ft"),
            Yard => String::from("yd"),
            Mile => String::from("mi"),
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

impl SiblingUnit for MetricUnit {
    fn smaller_unit(&self) -> Option<Unit> {
        match self {
            MetricUnit::Yoctometer => None,
            MetricUnit::Zeptometer => Some(Unit::Metric(MetricUnit::Yoctometer)),
            MetricUnit::Attometer => Some(Unit::Metric(MetricUnit::Zeptometer)),
            MetricUnit::Femtometer => Some(Unit::Metric(MetricUnit::Attometer)),
            MetricUnit::Picometer => Some(Unit::Metric(MetricUnit::Femtometer)),
            MetricUnit::Nanometer => Some(Unit::Metric(MetricUnit::Picometer)),
            MetricUnit::Micrometer => Some(Unit::Metric(MetricUnit::Nanometer)),
            MetricUnit::Millimeter => Some(Unit::Metric(MetricUnit::Micrometer)),
            MetricUnit::Centimeter => Some(Unit::Metric(MetricUnit::Millimeter)),
            MetricUnit::Decimeter => Some(Unit::Metric(MetricUnit::Centimeter)),
            MetricUnit::Meter => Some(Unit::Metric(MetricUnit::Decimeter)),
            MetricUnit::Decameter => Some(Unit::Metric(MetricUnit::Meter)),
            MetricUnit::Hectometer => Some(Unit::Metric(MetricUnit::Decameter)),
            MetricUnit::Kilometer => Some(Unit::Metric(MetricUnit::Hectometer)),
            MetricUnit::Megameter => Some(Unit::Metric(MetricUnit::Kilometer)),
            MetricUnit::Gigameter => Some(Unit::Metric(MetricUnit::Megameter)),
            MetricUnit::Terameter => Some(Unit::Metric(MetricUnit::Gigameter)),
            MetricUnit::Petameter => Some(Unit::Metric(MetricUnit::Terameter)),
            MetricUnit::Exameter => Some(Unit::Metric(MetricUnit::Petameter)),
            MetricUnit::Zettameter => Some(Unit::Metric(MetricUnit::Exameter)),
            MetricUnit::Yottameter => Some(Unit::Metric(MetricUnit::Zettameter)),
        }
    }

    fn greater_unit(&self) -> Option<Unit> {
        match self {
            MetricUnit::Yoctometer => Some(Unit::Metric(MetricUnit::Zeptometer)),
            MetricUnit::Zeptometer => Some(Unit::Metric(MetricUnit::Attometer)),
            MetricUnit::Attometer => Some(Unit::Metric(MetricUnit::Femtometer)),
            MetricUnit::Femtometer => Some(Unit::Metric(MetricUnit::Picometer)),
            MetricUnit::Picometer => Some(Unit::Metric(MetricUnit::Nanometer)),
            MetricUnit::Nanometer => Some(Unit::Metric(MetricUnit::Micrometer)),
            MetricUnit::Micrometer => Some(Unit::Metric(MetricUnit::Millimeter)),
            MetricUnit::Millimeter => Some(Unit::Metric(MetricUnit::Centimeter)),
            MetricUnit::Centimeter => Some(Unit::Metric(MetricUnit::Decimeter)),
            MetricUnit::Decimeter => Some(Unit::Metric(MetricUnit::Meter)),
            MetricUnit::Meter => Some(Unit::Metric(MetricUnit::Decameter)),
            MetricUnit::Decameter => Some(Unit::Metric(MetricUnit::Hectometer)),
            MetricUnit::Hectometer => Some(Unit::Metric(MetricUnit::Kilometer)),
            MetricUnit::Kilometer => Some(Unit::Metric(MetricUnit::Megameter)),
            MetricUnit::Megameter => Some(Unit::Metric(MetricUnit::Gigameter)),
            MetricUnit::Gigameter => Some(Unit::Metric(MetricUnit::Terameter)),
            MetricUnit::Terameter => Some(Unit::Metric(MetricUnit::Petameter)),
            MetricUnit::Petameter => Some(Unit::Metric(MetricUnit::Exameter)),
            MetricUnit::Exameter => Some(Unit::Metric(MetricUnit::Zettameter)),
            MetricUnit::Zettameter => Some(Unit::Metric(MetricUnit::Yottameter)),
            MetricUnit::Yottameter => None,
        }
    }
}

impl ToString for MetricUnit {
    fn to_string(&self) -> String {
        match self {
            Yoctometer => String::from("ym"),
            Zeptometer => String::from("zm"),
            Attometer => String::from("am"),
            Femtometer => String::from("fm"),
            Picometer => String::from("pm"),
            Nanometer => String::from("nm"),
            Micrometer => String::from("µm"),
            Millimeter => String::from("mm"),
            Centimeter => String::from("cm"),
            Decimeter => String::from("dm"),
            Meter => String::from("m"),
            Decameter => String::from("dam"),
            Hectometer => String::from("hm"),
            Kilometer => String::from("km"),
            Megameter => String::from("Mm"),
            Gigameter => String::from("Gm"),
            Terameter => String::from("Tm"),
            Petameter => String::from("Pm"),
            Exameter => String::from("Em"),
            Zettameter => String::from("Zm"),
            Yottameter => String::from("Ym"),
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
            Unit::Astronomic(astronomic_unit) => astronomic_unit.to_string(),
            Unit::Imperial(imperial_unit) => imperial_unit.to_string(),
            Unit::Metric(metric_unit) => metric_unit.to_string(),
        }
    }
}
