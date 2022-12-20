use std::str::FromStr;
use std::{env, fs};

enum BattleResult {
    Win,
    Wrong,
    Draw,
}

impl BattleResult {
    pub fn score(self) -> i8 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Wrong => 0,
        }
    }

    pub fn battle(user: RockPaperScissor, enemy: RockPaperScissor) -> Self {
        if user == enemy.weak_against() {
            return Self::Win;
        }

        if user == enemy {
            return Self::Draw;
        }

        return Self::Wrong;
    }

    pub fn convert_to_part_2_rule(user: RockPaperScissor) -> Self {
        match user {
            RockPaperScissor::Rock => Self::Wrong,
            RockPaperScissor::Paper => Self::Draw,
            RockPaperScissor::Scissors => Self::Win,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RockPaperScissor {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissor {
    fn score(self) -> i8 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn weak_against(self) -> Self {
        match self {
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn strong_against(self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Rock => Self::Scissors,
            Self::Scissors => Self::Paper,
        }
    }

    fn battle_with(user: Self, enemy: Self) -> i64 {
        let result = BattleResult::battle(user, enemy).score();
        (result + user.score()) as i64
    }

    fn battle_with_part_2_rules(user: Self, enemy: Self) -> i64 {
        let user = match BattleResult::convert_to_part_2_rule(user) {
            BattleResult::Draw => enemy,
            BattleResult::Win => enemy.weak_against(),
            BattleResult::Wrong => enemy.strong_against(),
        };

        Self::battle_with(user, enemy)
    }

    pub fn battle_with_part_1_rules_from_str(contents: String) -> i64 {
        contents
            .lines()
            .map(|l| {
                let data: Vec<Self> = l.split(' ').map(|v| Self::from_str(v).unwrap()).collect();
                Self::battle_with(data[1], data[0])
            })
            .sum::<i64>()
    }

    pub fn battle_with_part_2_rules_from_str(contents: String) -> i64 {
        contents
            .lines()
            .map(|l| {
                let data: Vec<Self> = l.split(' ').map(|v| Self::from_str(v).unwrap()).collect();
                Self::battle_with_part_2_rules(data[1], data[0])
            })
            .sum::<i64>()
    }
}

impl FromStr for RockPaperScissor {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, ()> {
        match str {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).unwrap();
    let data = RockPaperScissor::battle_with_part_2_rules_from_str(contents);

    println!("Hello, world! {data:?}");
}

#[cfg(test)]
mod rock_paper_scissors_tests {
    use crate::RockPaperScissor;

    #[test]
    fn battle_with_part_1_rules_from_str() {
        let input = "A Y\nB X\nC Z".to_string();
        assert_eq!(
            RockPaperScissor::battle_with_part_1_rules_from_str(input),
            15
        );
    }

    #[test]
    fn battle_with_part_2_rules_from_str() {
        let input = "A Y\nB X\nC Z".to_string();
        assert_eq!(
            RockPaperScissor::battle_with_part_2_rules_from_str(input),
            12
        );
    }
}
