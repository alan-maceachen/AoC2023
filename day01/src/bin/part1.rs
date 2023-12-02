fn main() {
    let calibration_sum = include_str!("../../input.txt")
        .lines()
        .map(|letters| letters.chars().filter(|c| c.is_ascii_digit()))
        .map(|mut digits| {
            let first = digits.next().expect("No digits found");
            let last = digits.last().unwrap_or(first);
            format!("{first}{last}")
                .parse::<u16>()
                .expect("Failed to parse")
        })
        .sum::<u16>();

    println!("Sum of calibration values: {:?}", calibration_sum);
}
