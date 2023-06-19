/// This is the offset that needs to be applied to K to get C
///   - C = K - 273   <-- This is the 273
pub const CELCIUS_KELVIN_OFFSET: i32 = 273;

/// This is the factor that needs to be applied to C to convert to F
///   - F = (1.8*(K-273)) + 32   <-- This is the 1.8
pub const FAHRENHEIT_CONVERSION_FACTOR: f64 = 1.8;

/// This is the offset that needs to be added to the temperature to convert from K to F
///   - F = (1.8*(K-273)) + 32   <-- This is the 32
pub const FAHRENHEIT_OFFEST: i32 = 32;
