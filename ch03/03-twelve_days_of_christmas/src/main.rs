fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 0..days.len() {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[day]
        );
        if day == 0 {
            println!("A partridge in a pear tree.\n");
            continue;
        }
        for day in (0..=day).rev() {
            match day {
                0 => println!("And a partridge in a pear tree.\n"),
                1 => println!("Two turtle doves,"),
                2 => println!("Three French hens,"),
                3 => println!("Four calling birds,"),
                4 => println!("Five gold rings,"),
                5 => println!("Six geese a laying,"),
                6 => println!("Seven swans a swimming,"),
                7 => println!("Eight maids a milking,"),
                8 => println!("Nine drummers drumming,"),
                9 => println!("Ten pipers piping,"),
                10 => println!("Eleven ladies dancing,"),
                11 => println!("Twelve Lords a leaping,"),
                _ => println!("wat"),
            };
        }
    }
}
