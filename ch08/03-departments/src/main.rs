use std::collections::HashMap;
use std::io;

fn main() {
    let mut workers_departments: HashMap<String, String> = HashMap::new();

    println!("{:?}", workers_departments);

    println!("CLI Department Management v0.1\n");
    println!("Menu (number corresponds to action):");
    println!("1) Add a person to a department");
    println!("2) List people in the department");
    println!("3) List people in company by department");
    println!("4) Exit the program\n");
    loop {
        println!("Please enter your input: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => add_x_to_dep(&mut workers_departments),
            2 => list_dep(&workers_departments),
            3 => list_all_by_deps(&workers_departments),
            4 => break,
            _ => continue,
        };
    }
}

fn add_x_to_dep(map: &mut HashMap<String, String>) {
    let mut input = String::new();

    println!("Enter the name: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    let key = input.trim().to_string();

    input.clear();
    println!("Enter the department: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let value = input.trim().to_string();

    map.insert(key, value);
    // map.insert("John".to_string(), "Daycare".to_string());
}

fn list_dep(map: &HashMap<String, String>) {
    let mut vec: Vec<String> = Vec::new();

    for (_, value) in map {
        vec.push(value.trim().to_string());
    }

    vec.sort();
    vec.dedup();

    println!("Choose a department:");

    for i in 0..vec.len() {
        println!("{}) {}", i + 1, &vec[i]);
    }

    loop {
        println!("Choose the department: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input > 0 && input <= vec.len() as u32 {
            for (key, value) in map {
                if value == &vec[(input - 1) as usize] {
                    println!("{}: {}", key, value);
                };
            }
            break;
        } else {
            continue;
        };
    }
}

fn list_all_by_deps(map: &HashMap<String, String>) {
    let mut vec: Vec<(String, String)> = Vec::new();
    for (key, value) in map {
        vec.push((value.to_string(), key.to_string()));
    }

    vec.sort();

    for i in vec {
        println!("{}: {}", i.0, i.1);
    }
}
