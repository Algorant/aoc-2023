use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;


fn main() {
    let path = Path::new("/home/ivan/dev/git/aoc-2023/day-01/input1.txt");
    let file = File::open(&path).expect("Could not open file");
    let reader = BufReader::new(file);


    let digit_pairs = extract_integers(reader);

    // Transform digit pairs into two-digit integers
    let two_digit_integers = digit_pairs.into_iter()
        .map(|(first_digit, last_digit)| {
            format!("{}{}", first_digit, last_digit).parse::<i32>().unwrap()
        })
        .collect::<Vec<i32>>();

    // Calculate the sum of all two-digit integers
    let sum: i32 = two_digit_integers.iter().sum();
    println!("Sum of all two-digit integers: {}", sum);

}


fn extract_integers(reader: BufReader<File>) -> Vec<(char, char)> {
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");

        // Filter out only the digits from the line
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

        // Check if there are any digits in the line
        if !digits.is_empty() {
            let first_digit = *digits.first().unwrap();
            let last_digit = *digits.last().unwrap();

            pairs.push((first_digit, last_digit));
        }
    }

    pairs
}

