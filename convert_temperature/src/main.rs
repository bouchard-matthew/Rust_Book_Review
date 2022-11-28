use std::io;

fn main() {
    // Getting user input for temp type they wish to convert from
    println!("Please enter the temperature type you wish to convert from (C, F, or K): ");

    let mut from = String::new();

    io::stdin().read_line(&mut from).expect("Unable to read line");

    from = from.trim().to_string();

    // Getting user input for temp type they wish to convert to
    println!("Please enter the temperature type you wish to convert to (C, F, or K): ");

    let mut to = String::new();

    io::stdin().read_line(&mut to).expect("Unable to read line");

    to = to.trim().to_string();

    // Getting user input for temp value to convert
    println!("Please enter the temperature value you wish to convert: ");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Unable to read line");

    let temp = match temp.trim().parse::<f32>() {
        Ok(num) => convert(from, to, num),
        Error => println!("Failed"),
    };
    
}

/* 
Formulas: 
    F to C = (F - 32) * 5/9
    F to K = (F + 459.67) * 5/9
    C to F = (C + 32) * 9/5
    C to K = C + 273.15
    K to F = ((K − 273.15) * 9/5) + 32
    K to C = K - 273.15
*/

fn convert(from: String, to: String, temp: f32) {
    match from {
        _ if from == "F" => convert_from_fahrenheit(to, temp),
        _ if from == "C" => convert_from_celcius(to, temp),
        _ if from == "K" => convert_from_kelvin(to, temp),
        _ => println!("Did not enter valid data")
    }
}

fn convert_from_fahrenheit(to: String, temp: f32) {
    match to {
        _ if to == "K" => println!("{}° Fahrenheit is equal to {}° Kelvin", temp, ((temp + 459.67) * 5.00)/9.00),
        _ if to == "C" => println!("{}° Fahrenheit is equal to {}° Celcius", temp, ((temp - 32.00) * 5.00)/9.00),
        _ => println!("Did not enter valid data"),
    }
}

fn convert_from_celcius(to: String, temp: f32) {
    match to {
        _ if to == "F" => println!("{}° Celcius is equal to {}° Fahrenheit", temp, ((temp + 32.00) * 9.00)/5.00),
        _ if to == "K" => println!("{}° Celcius is equal to {}° Kelvin", temp, temp + 273.15),
        _ => println!("Did not enter valid data"),
    }
}

fn convert_from_kelvin(to: String, temp: f32) {
    match to {
        _ if to == "F" => println!("{}° Kelvin is equal to {}° Fahrenheit", temp, (((temp - 273.15) * 9.00)/5.00) + 32.00),
        _ if to == "C" => println!("{}° Kelvin is equal to {}° Celcius", temp, temp - 273.15),
        _ => println!("Did not enter valid data"),
    }
}