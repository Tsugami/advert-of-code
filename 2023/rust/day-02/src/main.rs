use std::str::FromStr;

type Result<T> = std::result::Result<T, GameError>;

fn main() {
    println!("Hello, world!");
}

#[derive(Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(Clone)]
struct Cube {
    color: Color,
    value: usize,
}

#[derive(Debug)]
enum GameError {
    ParseCubeError(String),
    ParseCubeColorError(String),
    ParseCubeNumberError(String),
    SplitRoundLineError(String),
    SplitNumberRoundLineError(String),
}

impl FromStr for Cube {
    type Err = GameError;

    fn from_str(s: &str) -> Result<Self> {
        let (number, color) = s
            .trim()
            .split_once(" ")
            .ok_or(GameError::ParseCubeError(s.to_string()))?;

        let value = number
            .parse::<usize>()
            .ok()
            .ok_or(GameError::ParseCubeError(number.to_string()))?;

        let color = match color {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            color => return Err(GameError::ParseCubeColorError(color.to_string())),
        };

        Ok(Cube { color, value })
    }
}

#[derive(PartialEq, Debug, Clone)]
struct GameOptions {
    blue: usize,
    red: usize,
    green: usize,
}

impl std::ops::Add<Cube> for GameOptions {
    type Output = GameOptions;

    fn add(mut self, rhs: Cube) -> Self::Output {
        match rhs.color {
            Color::Blue => {
                self.blue += rhs.value;
                self
            }
            Color::Green => {
                self.green += rhs.value;
                self
            }
            Color::Red => {
                self.red += rhs.value;
                self
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Game {
    number: usize,
    result: GameOptions,
}

fn parse_game(game_str: &str) -> Result<Game> {
    let (gameNumber, rounds) = game_str
        .trim()
        .split_once(":")
        .ok_or(GameError::SplitRoundLineError(game_str.to_string()))?;

    let number = gameNumber
        .trim()
        .split_once(" ")
        .and_then(|(_, number)| number.parse::<usize>().ok())
        .ok_or(GameError::SplitNumberRoundLineError(gameNumber.to_string()))?;

    let rounds = rounds
        .split(";")
        .map(|round| {
            round
                .split(",")
                .map(|cube| Cube::from_str(cube))
                .collect::<Result<Vec<Cube>>>()
        })
        .collect::<Result<Vec<Vec<Cube>>>>()?;

    let result = rounds.iter().fold(
        GameOptions {
            blue: 0,
            green: 0,
            red: 0,
        },
        |mut game, cubes| {
            for cube in cubes {
                game = game + cube.clone();
            }

            game
        },
    );

    Ok(Game { number, result })
}

fn part1(input: String, gameConfig: GameOptions) -> Result<usize> {
    let games = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_game(line))
        .collect::<Result<Vec<Game>>>()?;

    let result = games
        .iter()
        .filter(|game| {
            game.result.green <= gameConfig.green
                && game.result.red <= gameConfig.red
                && game.result.blue <= gameConfig.blue
        })
        .fold(0, |acc, cur| cur.number + acc);

    Ok(result)
}

#[test]
fn test_part_1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    assert_eq!(
        part1(
            input.to_string(),
            GameOptions {
                blue: 14,
                red: 12,
                green: 13
            }
        )
        .unwrap(),
        8
    );
}

#[test]
fn test_parse_game() {
    assert_eq!(
        parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
        Game {
            number: 1,
            result: GameOptions {
                blue: 9,
                red: 5,
                green: 4
            }
        }
    )
}
