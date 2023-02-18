use std::io;

fn main() {
    let fib_n = loop {
        let mut fib_n = String::new();
        println!("Input the n of fibonacci number");

        if let Err(_) = io::stdin().read_line(&mut fib_n) {
            println!("Error reading the line, try again.");
            continue;
        };

        match fib_n.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    fibonacci(fib_n);

    println!("The {}th Fibonacci number is {}.", fib_n, fibonacci(fib_n));
}

fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }

    let mut f: u64 = 0;

    let mut i = 0;

    let mut f1 = 1;
    let mut f2 = 1;

    while i < (n - 2) {
        f = f1 + f2;
        f1 = f2;
        f2 = f;

        i += 1;
    }

    f
}
