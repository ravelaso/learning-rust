#[derive(Debug, Clone, Copy)]
enum TempUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

fn parse_unit(s: &str) -> Result<TempUnit, String> {
    match s {
        "C" => Ok(TempUnit::Celsius),
        "F" => Ok(TempUnit::Fahrenheit),
        "K" => Ok(TempUnit::Kelvin),
        _ => Err(format!("Unkown unit: {s}")),
    }
}

fn convert(value: f64, from: TempUnit, to: TempUnit) -> f64 {
    let celsius = match from {
        TempUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TempUnit::Kelvin => value - 273.15,
        TempUnit::Celsius => value,
    };
    match to {
        TempUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TempUnit::Kelvin => celsius + 273.15,
        TempUnit::Celsius => celsius,
    }
}

pub fn parse_f_to_c(input: f64) -> Result<(), String> {
    let from = parse_unit("F")?;
    let to = parse_unit("C")?;
    let result = convert(input, from, to);
    println!("{input}°F = {result:.1}°C");
    Ok(())
}
