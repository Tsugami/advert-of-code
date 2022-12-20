// uma stack é movida para uma diferente stack

use std::{fmt::Error, str::FromStr};

fn main() {
    println!("Hello, world!");
}

type Grid = Vec<Vec<String>>;

#[derive(Debug)]
struct Procedure {
    from: i64,
    to: i64,
    count: i64,
}

enum CratesError {
    InvalidProcedureLineFormat,
}

impl Procedure {
    fn from_str(s: &str) -> Option<Procedure> {
        let s = s.trim();

        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"move\s(\w*)\sfrom\s(\w*)\sto\s(\w*)").unwrap();
        }

        let caps = RE.captures(s)?;
        let count = caps.get(0)?.as_str().parse::<i64>().ok()?;
        let from = caps.get(1)?.as_str().parse::<i64>().ok()?;
        let to = caps.get(2)?.as_str().parse::<i64>().ok()?;

        Some(Procedure { from, to, count })
    }
}

// impl FromStr for Procedure {
//     type Err = CratesError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {

//         // if !s.starts_with("move") {
//         //     return Err("invalid pattern: \"move $ from $ to $\"".to_string());
//         // }

//         let d: Option<(i64, i64, i64)> = {

//             (count, from, to)
//         };

//         println!("alo {s}");
//         return todo!();
//         // return Err("ops".to_string());
//         Ok(Self {
//             count: 1,
//             from: 1,
//             to: 1,
//         })
//     }
// }

fn separate_crates_and_produce(
    contents: String,
) -> Result<(Vec<String>, Vec<String>), &'static str> {
    let mut crates = Vec::new();
    let mut procedure = Vec::new();

    let mut iter = contents
        .lines()
        .filter(|str| !str.is_empty())
        .collect::<Vec<&str>>();

    iter.reverse();

    for i in iter {
        let a = Procedure::from_str(i);
        println!("{i} --- {a:?}");
        // if i.trim().is_empty() && is_crates_lines && !crates.is_empty() {
        //     is_crates_lines = false;
        // } else if i.trim(). {

        // } else if is_crates_lines {
        //     crates.push(i.to_string())
        // } else {
        //     procedure.push(i.trim().to_string())
        // }
    }
    println!("{crates:?}");
    // if is_crates_lines {
    //     return Err("must have an empty line to separate the crates and the procedure");
    // }

    Ok((crates, procedure))
}

fn process(contents: String) -> Result<String, &'static str> {
    let (crates, procedure) = separate_crates_and_produce(contents)?;
    println!("crates: {crates:?}\n procedure: {procedure:?}");
    Ok("".to_string())
}

#[cfg(test)]
mod test {
    use crate::process;

    #[test]
    fn test_example() {
        let input = "
        [D]    
        [N] [C]    
        [Z] [M] [P]
        1   2   3 
        
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
        ";

        assert_eq!(process(input.to_owned()), Ok("CMZ".to_string()))
    }
}
