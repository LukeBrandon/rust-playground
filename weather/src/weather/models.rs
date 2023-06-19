use crate::weather::constants::{
    CELCIUS_KELVIN_OFFSET, FAHRENHEIT_CONVERSION_FACTOR, FAHRENHEIT_OFFEST,
};
use serde::{Deserialize, Serialize};

/// Enumeration holding the possible weather units
pub enum WeatherUnits {
    Celcius,
    Fahrenheit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub temp_min: KelvinTemperature,
    pub temp_max: KelvinTemperature,
    pub feels_like: KelvinTemperature,
    pub humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWind {
    pub speed: f32,
    pub gust: Option<f32>,
}

/// Struct for weather data gotten from OpenWeatherMap
///   - https://openweathermap.org/current
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub main: CurrentWeather,
    pub wind: CurrentWind,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KelvinTemperature(pub f64);

/// Temperature Conversion trait, provides methods to do the following
///   - Convert to celcius
///   - Convert to fahrehneit
pub trait TemperatureConversion {
    fn to_fahrenheit(&self) -> f64;
    fn to_celcius(&self) -> f64;
}

impl TemperatureConversion for KelvinTemperature {
    fn to_fahrenheit(&self) -> f64 {
        return (FAHRENHEIT_CONVERSION_FACTOR * self.to_celcius()) + f64::from(FAHRENHEIT_OFFEST);
    }

    fn to_celcius(&self) -> f64 {
        return self.0 - f64::from(CELCIUS_KELVIN_OFFSET);
    }
}
