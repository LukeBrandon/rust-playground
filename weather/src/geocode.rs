use geocoding::{Forward, GeocodingError, Opencage, Point};
use std::env::{self, VarError};

#[derive(Debug)]
pub struct ConfigError(VarError);

#[derive(Debug)]
pub enum GeocodeError {
    ConfigError(ConfigError),
    GeocodingError(GeocodingError),
    NoResultsError,
}

impl From<GeocodingError> for GeocodeError {
    fn from(value: GeocodingError) -> Self {
        GeocodeError::GeocodingError(value)
    }
}

impl From<ConfigError> for GeocodeError {
    fn from(value: ConfigError) -> Self {
        GeocodeError::ConfigError(value)
    }
}

/// This function takes in a zip code, and attempts to geocode it into a lat,lng.
/// This function can error if:
///   - There is an invalid opencage api key -> ConfigError
///   - Error from the opencage api, for various reasons -> GeocodingError
///   - There is was no results from opencage API -> NoResultsError
///
/// Output: A geocoding::Point struct or a Geocode error
fn geocode_opencage(zip_code: &String) -> Result<Point, GeocodeError> {
    // Initialize the Geocode client.
    let api_key = env::var("OPENCAGE_KEY").map_err(|e| ConfigError(e))?;

    let oc = Opencage::new(api_key.to_string());

    let res: Vec<Point<f64>> = oc.forward(&zip_code)?;
    let first_result = res.get(0).ok_or(GeocodeError::NoResultsError)?;

    // info!("Got forward geocoding results -> {:?}", first_result);
    return Ok(*first_result);
}

pub fn opencage_geocode(zip: &String) -> Result<Point, GeocodeError> {
    return geocode_opencage(zip);
}
