fn main() {
    let sum = include_str!("../../input.txt")
        .lines()
        .map(|letters| {
            let first = find_digit(letters, false).expect("No first digit found");
            let last = find_digit(letters, true).expect("No last digit found");

            format!("{first}{last}")
                .parse::<u16>()
                .expect("Failed to parse digits")
        })
        .sum::<u16>();

    println!("Sum of calibration values: {:?}", sum);
}

fn find_digit(word: &str, reverse: bool) -> Option<char> {
    let word = if reverse {
        word.chars().rev().collect::<String>()
    } else {
        word.to_string()
    };
    let mut potential_word = String::new();
    for c in word.chars() {
        if c.is_ascii_digit() {
            return Some(c);
        }
        potential_word.push(c);
        let word_to_check = if reverse {
            potential_word.chars().rev().collect::<String>()
        } else {
            potential_word.clone()
        };
        if let Some(digit) = substring_includes_digit(&word_to_check) {
            return Some(digit);
        }
    }
    None
}

const DIGITS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn substring_includes_digit(word: &str) -> Option<char> {
    DIGITS
        .iter()
        .find(|&&digit| word.contains(digit))
        .and_then(|&word| word_to_char_if_digit(word))
}

fn word_to_char_if_digit(word: &str) -> Option<char> {
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}
