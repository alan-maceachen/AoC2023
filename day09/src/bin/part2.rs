use std::vec;

use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn main() {
    let historic_sum: i64 = include_str!("../../input.txt")
        .lines()
        .map(|line| parse_sequence(line).expect("Unable to parse line").1)
        .map(|sequence| {
            let mut sequence = sequence;
            let mut first_numbers = vec![];
            while !sequence.final_value() {
                let first_number = *sequence.0.first().expect("No first element");
                first_numbers.push(first_number);
                sequence = sequence.next_sequence();
            }
            first_numbers.iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum();

    println!("Sum of historic numbers: {}", historic_sum);
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn final_value(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    fn next_sequence(&self) -> Self {
        Sequence(
            self.0
                .iter()
                .tuple_windows()
                .map(|(a, b)| *b - *a)
                .collect(),
        )
    }
}

// nom parser
fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    let (input, sequence) = separated_list1(tag(" "), nom::character::complete::i64)(input)?;
    Ok((input, Sequence(sequence)))
}
