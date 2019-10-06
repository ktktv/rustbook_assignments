fn main() {
    let hello = String::from("hello world what a beautiful day it is");

    let split = hello.split(" ");

    let vec = split.collect::<Vec<&str>>();

    for i in vec.iter() {
        print!("{} ", pig_latin(i));
    }
}

fn pig_latin(string: &str) -> String {
    let s = String::from(&string[1..string.len()]);

    match &string[0..1] {
        "b" => s + "-bay",
        "c" => s + "-cay",
        "d" => s + "-day",
        "f" => s + "-fay",
        "g" => s + "-gay",
        "h" => s + "-hay",
        "j" => s + "-jay",
        "k" => s + "-kay",
        "l" => s + "-lay",
        "m" => s + "-may",
        "n" => s + "-nay",
        "p" => s + "-pay",
        "q" => s + "-qay",
        "r" => s + "-ray",
        "s" => s + "-say",
        "t" => s + "-tay",
        "v" => s + "-vay",
        "w" => s + "-way",
        "x" => s + "-xay",
        "z" => s + "-zay",
        _ => String::from(string) + "-hay",
    }
}
