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
