use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    temp_min: f64,
    temp_max: f64,
    feels_like: f64,
    humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWind {
    speed: f32,
    gust: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    main: CurrentWeather,
    wind: CurrentWind,
}

async fn fetch_weather(lat: &f64, lon: &f64) -> WeatherData {
    let client: reqwest::Client = reqwest::Client::new();

    let api_key: String = env::var("WEATHER_KEY").unwrap();

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
        Err(_) => {
            panic!();
        }
    };

    let final_res: WeatherData = match str_res {
        Ok(r) => r,
        Err(_) => {
            panic!();
        }
    };

    return final_res;
}

pub fn weather_for_lat_lon(lat: &f64, lon: &f64) -> WeatherData {
    let rt: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let res: WeatherData = rt.block_on(async { fetch_weather(lat, lon).await });
    return res;
}
