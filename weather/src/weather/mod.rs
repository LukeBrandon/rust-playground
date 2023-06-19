// Only exposing the Weather Units enum
pub use crate::weather::models::WeatherUnits;

use crate::weather::models::{TemperatureConversion, WeatherData};
use std::env;

mod constants;
mod models;

async fn fetch_weather(lat: &f64, lon: &f64) -> WeatherData {
    let client: reqwest::Client = reqwest::Client::new();

    let api_key: String = env::var("WEATHER_KEY").expect("No weather API key provided");

    // API Reference:  https://openweathermap.org/current
    let request_url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_key}",
        lat = lat,
        lon = lon,
        api_key = api_key
    );

    let res: Result<reqwest::Response, reqwest::Error> = client.get(request_url).send().await;
    let str_res: Result<WeatherData, reqwest::Error> = match res {
        Ok(res) => res.json::<WeatherData>().await,
        Err(e) => {
            dbg!(e);
            panic!();
        }
    };

    let final_res: WeatherData = match str_res {
        Ok(r) => r,
        Err(e) => {
            dbg!(e);
            panic!();
        }
    };

    return final_res;
}

fn weather_for_lat_lon(lat: &f64, lon: &f64) -> WeatherData {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let res: WeatherData = rt.block_on(async { fetch_weather(lat, lon).await });
    return res;
}

/// Blockingly fetches and displays the weather information for the provided location.
/// Parameters:
///   - latitude (f64)
///   - longitued (f64)
///   - units (either "F" or "C" character)
///
/// Prints the following information:
///   - Minimum Temperature (either celcius or fahrenheit)
///   - Maximum Temperature (either celcius or fahrenheit)
///   -
pub fn get_and_display_weather(lat: &f64, lon: &f64, units: WeatherUnits) {
    let weather_data: WeatherData = weather_for_lat_lon(lat, lon);
    let units_char: char = match units {
        WeatherUnits::Celcius => 'C',
        WeatherUnits::Fahrenheit => 'F',
    };
    let min_temp = weather_data.main.temp_min.to_fahrenheit();
    let max_temp = weather_data.main.temp_min.to_fahrenheit();

    println!("Minimum Temperature: {:.1}{}", min_temp, units_char);
    println!("Maximum Temperature: {:.1}{}", max_temp, units_char);
}
