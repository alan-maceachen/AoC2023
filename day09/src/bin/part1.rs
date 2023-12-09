use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn main() {
    let prediction_sum = include_str!("../../input.txt")
        .lines()
        .map(|line| parse_sequence(line).expect("Unable to parse line").1)
        .map(|sequence| {
            let mut sequence = sequence;
            let mut prediction = 0;
            // Use the sum of the last numbers in the sequence to predict the next number
            while !sequence.final_value() {
                prediction += *sequence.0.last().expect("No last element");
                sequence = sequence.next_sequence();
            }
            prediction
        })
        .sum::<i64>();

    println!("Sum of next predicted numbers: {}", prediction_sum);
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

fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    let (input, sequence) = separated_list1(tag(" "), nom::character::complete::i64)(input)?;
    Ok((input, Sequence(sequence)))
}
