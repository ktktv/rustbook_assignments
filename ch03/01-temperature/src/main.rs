use std::io;

fn main() {
    loop {
        println!("Please input your temperature");

        let mut temperature = String::new();

        io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Inputted value is: {}", temperature); 

        let mut is_celsius = String::new();

        println!("Is the temperature in Celsius? (type true for celsius, false for fahrenheit)");

        io::stdin()
        .read_line(&mut is_celsius)
        .expect("Failed to read line");

        let is_celsius: bool = match is_celsius.trim().parse() {
            Ok(bul) => bul,
            Err(_) => continue,
        };

        if is_celsius == true {
            println!("Target temperature is {} degrees Fahrenheit.", convert(temperature, is_celsius));
        } else {
            println!("Target temperature is {} degrees Celsius.", convert(temperature, is_celsius));
        }

        break;
    }
}

fn convert (temp: f64, val: bool) -> f64 {
    match val {
        true => temp * 9.0/5.0 + 32.0,
        false => temp - 32.0 * 5.0/9.0,
    }
}