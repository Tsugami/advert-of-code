use std::{env, fs};

fn main() {
    let file_path = env::args()
        .nth(1)
        .expect("expect input file in first argument");

    let contents = fs::read_to_string(file_path).unwrap();

    match search_higher_calories(contents) {
        Ok(highest) => println!("highest value is {}", highest),
        Err(HigherCaloriesError::EmptyInput) => println!("The file is empty"),
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

fn search_higher_calories(contents: String) -> Result<i64, HigherCaloriesError> {
    let mut higher = 0;

    if contents.is_empty() {
        return Err(HigherCaloriesError::EmptyInput);
    }
    let mut line_number = 0;
    let mut current_amount = 0;

    for line in contents.lines() {
        line_number += 1;

        let line = line.trim();

        if line.is_empty() {
            if current_amount > higher {
                higher = current_amount;
            }

            current_amount = 0;
            continue;
        }

        match line.parse::<i64>() {
            Ok(value) => current_amount += value,
            _ => {
                return Err(HigherCaloriesError::InvalidLineFormat {
                    line_number,
                    contents: line.to_string(),
                });
            }
        }
    }

    Ok(higher)
}

#[cfg(test)]
mod search_higher_calories_tests {
    use crate::{search_higher_calories, HigherCaloriesError};

    #[test]
    fn empty_input_error() {
        let input = "".to_string();
        assert_eq!(
            search_higher_calories(input),
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
            search_higher_calories(input),
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

        assert_eq!(search_higher_calories(input), Ok(8000));
    }
}
