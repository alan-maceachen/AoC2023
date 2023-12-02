use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;

fn main() {
    let valid_games: u32 = include_str!("../../input.txt")
        .lines()
        .map(parse_game)
        .map(|game| game.expect("Failed to parse games").1)
        .filter_map(valid_game_id)
        .sum();

    println!("Valid games id's sum: {}", valid_games);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Colour {
    Blue,
    Green,
    Red,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Cubes {
    colour: Colour,
    amount: u32,
}

#[derive(Debug)]
struct Handful(Vec<Cubes>);

impl Handful {
    fn number_cubes(&self, colour: Colour) -> usize {
        self.0
            .iter()
            .find(|&&c| c.colour == colour)
            .map(|c| c.amount as usize)
            .unwrap_or(0)
    }

    fn valid_handful(&self) -> bool {
        self.number_cubes(Colour::Red) <= 12
            && self.number_cubes(Colour::Green) <= 13
            && self.number_cubes(Colour::Blue) <= 14
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    handfuls: Vec<Handful>,
}

fn valid_game_id(game: Game) -> Option<u32> {
    if game.handfuls.iter().all(|handful| handful.valid_handful()) {
        Some(game.id)
    } else {
        None
    }
}

fn parse_number_and_cube(input: &str) -> nom::IResult<&str, Cubes> {
    let (input, amount) = map_res(digit1, str::parse)(input)?;

    let (input, _) = space1(input)?;

    let (input, colour) = alt((
        map(tag("red"), |_| Colour::Red),
        map(tag("green"), |_| Colour::Green),
        map(tag("blue"), |_| Colour::Blue),
    ))(input)?;

    Ok((input, Cubes { amount, colour }))
}

fn parse_handful(input: &str) -> nom::IResult<&str, Handful> {
    let (input, cubes) = separated_list1(tag(", "), parse_number_and_cube)(input)?;

    Ok((input, Handful(cubes)))
}

fn parse_game(input: &str) -> nom::IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, handfuls) = all_consuming(separated_list1(tag("; "), parse_handful))(input)?;

    Ok((input, Game { id, handfuls }))
}
