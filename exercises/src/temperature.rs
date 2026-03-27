use core::f64;
use std::sync::atomic::{AtomicU32, Ordering};

const ABSOLUTE_ZERO_C: f64 = -273.15;
static CONVERSION_COUNT: AtomicU32 = AtomicU32::new(0);

fn celsius_to_fahrenheit(c: f64) -> f64{
    if c < ABSOLUTE_ZERO_C {
        return f64::NAN;
    }
    CONVERSION_COUNT.fetch_add(1, Ordering::Relaxed);
    c * 9.0 / 5.0 + 32.0 //Conversion
}

pub fn test(){
    let temp = "98.6";           // &str
    let temp: f64 = temp.parse().unwrap(); // shadow as f64
    let temp = celsius_to_fahrenheit(temp); // shadow as Fahrenheit
    println!("Converting from: 98.6°C");
    println!("Result: {temp:.1}°F");
    println!("Conversions: {}", CONVERSION_COUNT.load(Ordering::Relaxed));
}

