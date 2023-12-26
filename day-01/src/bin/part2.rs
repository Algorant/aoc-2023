// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let path = Path::new("/home/ivan/dev/aoc-2023/day-01/input1.txt");
    // let file = File::open(&path).expect("Could not open file");
    // let reader = BufReader::new(file);
    let input = std::fs::read_to_string(path).unwrap(); 
    let lines = input.lines().collect::<Vec<_>>();

    println!("part2: {}", part2(&lines));



}

// fn create_digit_mappings() -> HashMap<String, String> {
//     let mut mappings = HashMap::new();
//     mappings.insert("one".to_string(), "1".to_string());
//     mappings.insert("two".to_string(), "2".to_string());
//     mappings.insert("three".to_string(), "3".to_string());
//     mappings.insert("four".to_string(), "4".to_string());
//     mappings.insert("five".to_string(), "5".to_string());
//     mappings.insert("six".to_string(), "6".to_string());
//     mappings.insert("seven".to_string(), "7".to_string());
//     mappings.insert("eight".to_string(), "8".to_string());
//     mappings.insert("nine".to_string(), "9".to_string());
//     mappings
// }

fn has_prefixes(line: &str) -> (char, bool) {
    let prefixes = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ];
    for (prefix, character) in prefixes {
        if line.starts_with(prefix) {
            return (character, true);
        }
    }
    (' ', false)
}

fn find_first_digit(chars: &[char], traversal: &[usize]) -> Option<char> {
    for i in traversal {
        if chars[*i].is_ascii_digit() {
            return Some(chars[*i]);
        }
        let (c, found) = has_prefixes(&chars[*i..].iter().collect::<String>());
        if found {
            return Some(c);
        }
    }
    None
    
}

fn part2(lines: &[&str]) -> u32 {
    let mut total = 0;

    for line in lines {
    // find first and last digits in the string.
    let chars = line.chars().collect::<Vec<_>>();
    let first = find_first_digit(&chars, &(0..chars.len()).collect::<Vec<_>>()).unwrap();
    let last = find_first_digit(&chars, &(0..chars.len()).rev().collect::<Vec<_>>()).unwrap();

    let first = first.to_digit(10).unwrap();
    let last = last.to_digit(10).unwrap();
    let number = first * 10 + last;
    total += number;

    }

    total
}
