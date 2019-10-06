use std::collections::HashMap;

fn main() {
    let numbers: Vec<i32> = vec![
        71, 3, 91, 63, 75, 47, 46, 94, 9, 50, 95, 24, 36, 24, 64, 28, 75, 31, 7, 33, 81, 50, 6, 43,
        87, 14, 49, 10, 2, 74, 71, 13, 61, 64, 24,
    ];

    println!("Mean is: {}", mean(&numbers));
    println!("Median is: {}", median(&numbers));
    println!("Mode is: {}", mode(&numbers));
}

fn mean(vec: &[i32]) -> i32 {
    let mut a: i32 = 0;

    for i in vec.iter() {
        a += i;
    }

    a / vec.len() as i32
}

fn median(vec: &[i32]) -> i32 {
    let mut v = Vec::from(vec);
    v.sort();

    v[v.len() / 2 - 1]
}

fn mode(vec: &[i32]) -> i32 {
    let mut mode = HashMap::new();

    for i in vec.iter() {
        let count = mode.entry(i).or_insert(0);
        *count += 1;
    }

    let mut mode_vec: Vec<_> = mode.iter().collect();

    mode_vec.sort_by(|a, b| b.1.cmp(a.1));

    **mode_vec.first().unwrap().0
}
