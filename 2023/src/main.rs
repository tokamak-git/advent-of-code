use std::fs::File;
use std::io::*;
use std::io::{self, BufReader};
use std::option::Option;

fn main() -> io::Result<()> {
    // read file into chunks and process int from them using filtering logic
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let c1: u32;
        let c2: u32;
        let s = line.expect("unable to read line from file");
        match read_line(s.chars()) {
            Some(c) => c1 = c,
            None => panic!("Failed to find any digit in"),
        };
        match read_line(s.chars().rev()) {
            Some(c) => c2 = c,
            None => panic!("Failed to find any digit in"),
        };

        // construct number
        sum += format!("{}{}", c1, c2)
            .parse::<u32>()
            .expect("failed to find number");
    }
    // divide into chunks for parallel processing - streach goal
    // write filter logic
    // every sentence contains a single 2 digit number, defined as first and last number in
    // string
    println!("sum: {}", sum);
    Ok(())
}

fn read_line<I>(iter: I) -> Option<u32>
where
    I: IntoIterator<Item = char>,
{
    let mut num: String = String::new();
    for c in iter {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        }
        num.push_str(&c.to_string());

        match num.as_str() {
            "zero" => Some(0),
            "one" => {
                (println!("in one"));
                Some(1)
            }
            "two" => {
                (println!("in two"));
                Some(2)
            }
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => continue,
        };

        match num.as_str() {
            "orez" => Some(0),
            "eno" => Some(1),
            "owt" => {
                (println!("in owt"));
                Some(2)
            }
            "eerht" => Some(3),
            "ruof" => Some(4),
            "evif" => Some(5),
            "xis" => Some(6),
            "neves" => Some(7),
            "thgie" => Some(8),
            "enin" => Some(9),
            _ => continue,
        };
    }
    None
}
