use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::option::Option;

fn main() -> io::Result<()> {
    // read file into chunks and process int from them using filtering logic
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // divide into chunks for parallel processing - streach goal
    // write filter logic
    // every sentence contains a single 2 digit number, defined as first and last number in
    // string
    println!("sum: {}", calculate_sum(reader));
    Ok(())
}

fn calculate_sum<R: BufRead>(r: R) -> u32 {
    let mut sum = 0;

    for line in r.lines() {
        let mut c1: u32 = 0;
        let mut c2: u32 = 0;

        let l = line.unwrap();

        println!("pre process line: {}", l);
        let s: String = pre_process(&l);
        println!("post process line: {}", s);
        match read_line(s.chars()) {
            Some(c) => c1 = c,
            // None => panic!("Failed to find any digit in"),
            None => println!("s: {}", s),
        };
        match read_line(s.chars().rev()) {
            Some(c) => c2 = c,
            // None => panic!("Failed to find any digit in"),
            None => println!("s: {}", s),
        };

        // construct number
        sum += format!("{}{}", c1, c2)
            .parse::<u32>()
            .expect("failed to find number");
        println!(
            "line: {:?},s: {}, c1: {}, c2: {}, sum: {}",
            l, s, c1, c2, sum
        );
    }
    sum
}

fn pre_process(s: &String) -> String {
    let digits: std::collections::HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut st = String::new();
    if let Some(ch) = s.chars().next() {
        if ch.is_numeric() {
            st += &ch.to_string();
        } else {
            for (k, v) in digits.iter() {
                if s.starts_with(k) {
                    st += v;
                    break;
                }
            }
        }
        st = format!("{}{}", st, pre_process(&s[1..].to_string()));
    };
    st
}

fn read_line<I>(iter: I) -> Option<u32>
where
    I: IntoIterator<Item = char>,
{
    for c in iter {
        if c.is_numeric() {
            return Some(c.to_digit(10).unwrap());
        };
    }
    None
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_sum() {
        let file = File::open("test.txt").unwrap();
        let reader = BufReader::new(file);

        assert_eq!(calculate_sum(reader), 281);
    }
}
