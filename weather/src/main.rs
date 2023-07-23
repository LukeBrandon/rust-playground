use core::num::ParseIntError;
use dotenv::dotenv;
use std::{io, process::exit};
use weather::WeatherUnits;

mod geocode;
mod weather;

/// Prints the welcome message for the weather tool
fn print_welcome_message() {
    println!("----------------------------------------------------------------------\n");
    println!("    Welcome to Luke's Rust weather command line tool.\n");
    println!("    This tool was made to help Luke learn rust and use things like");
    println!("      - Async web requests");
    println!("      - Error handling");
    println!("      - Rust modules\n");
    println!("----------------------------------------------------------------------\n\n");
}

#[derive(Debug)]
enum UserInputError {
    FiveDigitError(String),
    StandardInputError(String),
}
/// Promps the user for a 5 digit zip code
///   - Must be 5 digits, or returns FiveDigitError
///   - Returns a StandardInputErrro(String) if stdin()::read_line encounters an error
///
/// Output: The 5 digit string, otherwise FiveDigitError
fn read_5_digit_user_input() -> Result<String, UserInputError> {
    let mut buffer: String = String::from("");
    println!("Enter your 5-digit zip code: ");

    io::stdin().read_line(&mut buffer).map_err(|_| {
        UserInputError::StandardInputError("Error reading standard in: {e}".to_string())
    })?; // Propogate None if fail

    let val: &str = buffer.trim();

    if val.len() != 5 {
        return Err(UserInputError::FiveDigitError(
            "Input does not contain five digits.".to_string(),
        ));
    }

    return Ok(val.to_string());
}

#[derive(Debug)]
enum InvalidZipError {
    UserInputError(UserInputError),
    NonNumericError(ParseIntError),
}

// This tells rust how to convert from a UserInputError into a InvalidZipError this
// is an idiomatic way of handling and extending errors allowing for use of the ? operator
impl From<UserInputError> for InvalidZipError {
    fn from(err: UserInputError) -> Self {
        InvalidZipError::UserInputError(err)
    }
}

/// The function reads user zip input doing the following
///   - Read's user input for a zip code
///   - Validates it is numeric
///   - Returns the valid zip as a string or None if invalid
///
/// Output: A 5 digit numeric string or None
fn read_user_zip_input() -> Result<String, InvalidZipError> {
    let five_digit_string: String = read_5_digit_user_input()?;

    match five_digit_string.parse::<i32>() {
        Ok(_) => Ok(five_digit_string),
        Err(e) => Err(InvalidZipError::NonNumericError(e)),
    }
}

fn main() {
    dotenv().ok();
    print_welcome_message();

    // Run until we get a `Ok()` from `read_user_zip_input()`
    let zip_code: String;
    loop {
        match read_user_zip_input() {
            Ok(r) => {
                zip_code = r;
                break;
            }
            Err(err) => {
                println!("Please provide valid input: {:?}\n\n", err)
            }
        }
    }

    println!("Getting weather information for zip code: {}", &zip_code);

    let res = geocode::opencage_geocode(&zip_code).unwrap_or_else(|e| {
        dbg!(e);
        println!("Was not able to geocode your provided Zip code, please try again...");
        exit(1);
    });

    println!("Geocoded result is {:#?}\n", res);

    // TODO: Implement user input or perferences for the units
    weather::get_and_display_weather(&res.0.y, &res.0.x, WeatherUnits::Celcius);
    weather::get_and_display_weather(&res.0.y, &res.0.x, WeatherUnits::Fahrenheit);
}
