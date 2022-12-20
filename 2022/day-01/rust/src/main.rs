use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("expect input file in first argument");

    let contents = fs::read_to_string(file_path).unwrap();

    match CalorieCounting::sum_top_three_calories(contents) {
        Ok(highest) => println!("highest value is {:?}", highest),
        Err(HigherCaloriesError::EmptyInput) => println!("Cannot open the file."),
        Err(HigherCaloriesError::InvalidLineFormat {
            contents,
            line_number,
        }) => {
            println!(
                "the line {} with value {} should be a number",
                line_number, contents
            )
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum HigherCaloriesError {
    EmptyInput,
    InvalidLineFormat { contents: String, line_number: i64 },
}

struct CalorieCounting {
    inventories: Vec<i64>,
}

impl CalorieCounting {
    fn new() -> Self {
        Self {
            inventories: Vec::new(),
        }
    }

    fn parse(contents: String) -> Result<Self, HigherCaloriesError> {
        let mut calorie_counting = CalorieCounting::new();

        if contents.is_empty() {
            return Err(HigherCaloriesError::EmptyInput);
        }

        let mut current_line = 0;
        let mut current_inventory: Vec<i64> = Vec::new();

        for line in contents.lines() {
            current_line += 1;

            let line = line.trim();

            if line.is_empty() {
                calorie_counting
                    .inventories
                    .push(current_inventory.iter().sum());

                current_inventory = Vec::new();
                continue;
            }

            match line.parse::<i64>() {
                Ok(value) => current_inventory.push(value),
                _ => {
                    return Err(HigherCaloriesError::InvalidLineFormat {
                        line_number: current_line,
                        contents: line.to_string(),
                    });
                }
            }
        }

        Ok(calorie_counting)
    }

    fn sort(mut self) -> Self {
        self.inventories.sort_by(|a, b| b.cmp(&a));
        self
    }

    fn highest(contents: String, take: usize) -> Result<i64, HigherCaloriesError> {
        Ok(Self::parse(contents)?
            .sort()
            .inventories
            .iter()
            .take(take)
            .sum::<i64>())
    }

    pub fn highest_calorie(contents: String) -> Result<i64, HigherCaloriesError> {
        Ok(Self::highest(contents, 1)?)
    }

    pub fn sum_top_three_calories(contents: String) -> Result<i64, HigherCaloriesError> {
        Ok(Self::highest(contents, 3)?)
    }
}

#[cfg(test)]
mod search_higher_calories_tests {
    use crate::{CalorieCounting, HigherCaloriesError};

    #[test]
    fn empty_input_error() {
        let input = "".to_string();
        assert_eq!(
            CalorieCounting::highest_calorie(input),
            Err(HigherCaloriesError::EmptyInput)
        );
    }

    #[test]
    fn invalid_line_error() {
        let input = "
        1000
        2000

        1000
        1a
        3000
        "
        .to_string();

        assert_eq!(
            CalorieCounting::highest_calorie(input),
            Err(HigherCaloriesError::InvalidLineFormat {
                line_number: 6,
                contents: "1a".to_string()
            })
        );
    }

    #[test]
    fn found_highest_value() {
        let input = "
        1000
        2000

        1000
        4000
        3000

        1000
        3000
        2000
        "
        .to_string();

        assert_eq!(CalorieCounting::highest_calorie(input), Ok(8000));
    }
    #[test]
    fn find_three_top_calories() {
        let input = "
        10

        40

        30

        400

        20
        "
        .to_string();

        assert_eq!(CalorieCounting::sum_top_three_calories(input), Ok(470));
    }
}
