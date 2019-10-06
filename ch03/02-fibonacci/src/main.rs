use std::io;

fn main() {
    loop {
        println!("Input the n of fibonacci number");

        let mut fib_n = String::new();

        io::stdin()
            .read_line(&mut fib_n)
            .expect("Error reading the string");

        let fib_n: u64 = match fib_n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        fibonacci(fib_n);

        println!("The {}th Fibonacci number is {}.", fib_n, fibonacci(fib_n));
        break;
    }
}

fn fibonacci(n: u64) -> u64 {
    let mut f: u64 = 0;

    if n == 0 {
    } else if n == 1 || n == 2 {
        f = 1;
    } else {
        let mut i = 0;

        let mut f1 = 1;
        let mut f2 = 1;

        while i < (n - 2) {
            f = f1 + f2;
            f1 = f2;
            f2 = f;

            i += 1;
        }
    }

    f
}
