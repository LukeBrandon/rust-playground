use std::env;

pub mod geocode {
    use geocoding::{Forward, Opencage, Point};
    use std::env;

    fn geocode_opencage(zip_code: &String) -> Point {
        // Initialize the Geocode client.
        let api_key = env::var("OPENCAGE_KEY").unwrap();

        let oc = Opencage::new(api_key.to_string());

        let res: Vec<Point<f64>> = oc.forward(&zip_code).unwrap();
        let first_result = res.get(0).unwrap();

        println!(
            "{longitude}, {latitude}",
            longitude = first_result.x(),
            latitude = first_result.y()
        );
        return *first_result;
    }

    pub fn opencage_geocode(zip: &String) -> Point {
        return geocode_opencage(zip);
    }
}

pub mod weather {
    use std::env;
    use serde::{Serialize, Deserialize};

    #[derive(Debug)]
    #[derive(Serialize, Deserialize)]
    pub struct CurrentWeather {
        temp_min: f64,
        temp_max: f64,
        feels_like: f64,
        humidity: i32,
    }

    #[derive(Debug)]
    #[derive(Serialize, Deserialize)]
    pub struct CurrentWind {
        speed: f32,
        gust: Option<f32>
    }

    #[derive(Debug)]
    #[derive(Serialize, Deserialize)]
    pub struct WeatherData {
        main: CurrentWeather,
        wind: CurrentWind
    }

    // async fn fetch_weather(lat: &f64, lon: &f64) -> Result<reqwest::Response, reqwest::Error> {
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

        println!("{}", request_url);

        let res: Result<reqwest::Response, reqwest::Error> = client.get(request_url).send().await;
        let str_res: Result<WeatherData, reqwest::Error> = match res {
            Ok(res) => res.json::<WeatherData>().await,
            Err(_) => {
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

        // return res;
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
}

/// Prints the welcome message for the weather tool
fn print_welcome_message() {
    println!("Welcome to Luke's Rust weather command line tool.\n");
}

fn process_zip() -> String {
    let args: Vec<String> = env::args().collect();

    let input = args.get(1);

    // This could also be done with .unwrap()
    if let Some(z) = input {
        z.to_string()
    } else {
        println!("No zip was provided, cancelling");
        panic!();
    }
}

use crate::weather::WeatherData;
fn main() {
    print_welcome_message();

    let zip = process_zip();
    println!("Getting weather information for zip: {}", &zip);

    let res = geocode::opencage_geocode(&zip);
    println!("Geocoded result is {:#?}", res);

    let weather_res: WeatherData = weather::weather_for_lat_lon(&res.0.y, &res.0.x);
    println!("Weather result is {:#?}", weather_res);

    // match weather_res {
    //     Ok(res) => {
    //         println!("{:?}", res);
    //     }
    //     Err(e) => {
    //         println!("{}", e);
    //     }
    // }
}
