use std::io;

fn main() {
    loop {
        let mut buf = String::new();

        println!("Please input your temperature");
        let temperature = loop {
            buf.clear();

            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line");

            if let Ok(temperature) = buf.trim().parse() {
                break temperature;
            } else {
                println!("Please input correct temperature");
                continue;
            };
        };

        println!("Inputted value is: {}", temperature);

        println!("Is the temperature in Celsius? (type true for celsius, false for fahrenheit)");

        let is_celsius = loop {
            buf.clear();

            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read line");

            if let Ok(is_celsius) = buf.trim().parse() {
                break is_celsius;
            } else {
                println!("Invalid answer!");
                continue;
            };
        };

        if is_celsius == true {
            println!(
                "Target temperature is {:.1} degrees Fahrenheit.",
                convert(temperature, is_celsius)
            );
        } else {
            println!(
                "Target temperature is {:.1} degrees Celsius.",
                convert(temperature, is_celsius)
            );
        }

        break;
    }
}

fn convert(temp: f64, val: bool) -> f64 {
    match val {
        true => temp * 9.0 / 5.0 + 32.0,
        false => temp - 32.0 * 5.0 / 9.0,
    }
}
