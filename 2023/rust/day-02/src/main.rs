use std::include_str;
use std::str::FromStr;

type Result<T> = std::result::Result<T, GameError>;

fn main() {
    let input = include_str!("../input.txt");

    let config = GameOptions {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = part1(input.to_string(), config);
    println!("part 1: {:?}", result);

    let result = part2(input.to_string());
    println!("part 2: {:?}", result);
}

#[derive(PartialEq, Debug, Clone)]
enum Color {
    Blue,
    Red,
    Green,
}

#[derive(PartialEq, Debug, Clone)]
struct Cube {
    color: Color,
    value: usize,
}

#[derive(PartialEq, Debug, Clone)]
enum GameError {
    ParseCubeError(String),
    ParseCubeColorError(String),
    ParseCubeNumberError(String),
    SplitRoundLineError(String),
    SplitNumberRoundLineError(String),
    Part2,
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
            .ok_or(GameError::ParseCubeNumberError(number.to_string()))?;

        let color = match color {
            "blue" => Color::Blue,
            "red" => Color::Red,
            "green" => Color::Green,
            color => return Err(GameError::ParseCubeColorError(color.to_string())),
        };

        Ok(Cube { color, value })
    }
}

#[derive(PartialEq, Debug, Clone, Default, Eq)]
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
    rounds: Vec<GameOptions>,
}

fn parse_game(game_str: &str) -> Result<Game> {
    let (game_number, rounds) = game_str
        .trim()
        .split_once(":")
        .ok_or(GameError::SplitRoundLineError(game_str.to_string()))?;

    let number = game_number
        .trim()
        .split_once(" ")
        .and_then(|(_, number)| number.parse::<usize>().ok())
        .ok_or(GameError::SplitNumberRoundLineError(
            game_number.to_string(),
        ))?;

    let rounds = rounds
        .split(";")
        .map(|round| {
            round.split(",").map(|cube| Cube::from_str(cube)).try_fold(
                GameOptions::default(),
                |mut acc, f| {
                    if let Ok(cube) = f {
                        acc = acc + cube;
                    }

                    Ok(acc)
                },
            )
        })
        .collect::<Result<Vec<GameOptions>>>()?;

    Ok(Game { number, rounds })
}

fn part1(input: String, game_config: GameOptions) -> Result<usize> {
    let games = input
        .clone()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_game(line))
        .collect::<Result<Vec<Game>>>()?;

    let result = games
        .iter()
        .filter(|game| {
            game.rounds.iter().all(|round| {
                round.blue <= game_config.blue
                    && round.red <= game_config.red
                    && round.green <= game_config.green
            })
        })
        .fold(0, |acc, cur| cur.number + acc);

    Ok(result)
}

fn get_high_round(rounds: &Vec<GameOptions>, color: Color) -> Option<GameOptions> {
    let mut rounds = rounds.clone();
    rounds.sort_by(|a, b| match color {
        Color::Blue => b.blue.cmp(&a.blue),
        Color::Green => b.green.cmp(&a.green),
        Color::Red => b.red.cmp(&a.red),
    });

    rounds.get(0).cloned()
}

fn part2(input: String) -> Result<usize> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map_while(|line| parse_game(line).ok())
        .map_while(|game| {
            let blue_round = get_high_round(&game.rounds, Color::Blue)?;
            let green_round = get_high_round(&game.rounds, Color::Green)?;
            let red_round = get_high_round(&game.rounds, Color::Red)?;

            Some(blue_round.blue * green_round.green * red_round.red)
        })
        .reduce(|acc, cur| acc + cur)
        .ok_or(GameError::Part2)
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
fn test_part_2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    assert_eq!(part2(input.to_string()).unwrap(), 2286);
}

#[test]
fn test_parse_game() {
    assert_eq!(
        parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
        Game {
            number: 1,
            rounds: vec![
                GameOptions {
                    blue: 3,
                    red: 4,
                    green: 0
                },
                GameOptions {
                    blue: 6,
                    red: 1,
                    green: 2
                },
                GameOptions {
                    blue: 0,
                    red: 0,
                    green: 2
                }
            ]
        }
    )
}
