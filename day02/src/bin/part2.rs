use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::combinator::{all_consuming, map, map_res};
use nom::multi::separated_list1;

fn main() {
    let valid_games: usize = include_str!("../../input.txt")
        .lines()
        .map(parse_game)
        .map(|game| game.expect("Failed to parse games").1)
        .map(game_powerset)
        .sum();

    println!("Games minimum valid powerset: {}", valid_games);
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
}

#[derive(Debug)]
struct Game {
    _id: u32,
    handfuls: Vec<Handful>,
}

impl Game {
    fn max_cubes(&self, colour: Colour) -> usize {
        self.handfuls
            .iter()
            .map(|handful| handful.number_cubes(colour))
            .max()
            .expect("Cubes should always be present")
    }
}

fn game_powerset(game: Game) -> usize {
    game.max_cubes(Colour::Red) * game.max_cubes(Colour::Green) * game.max_cubes(Colour::Blue)
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

    Ok((input, Game { _id: id, handfuls }))
}
