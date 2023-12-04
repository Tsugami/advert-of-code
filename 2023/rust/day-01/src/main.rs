use std::env;
use std::fs::File;
use std::io::{self, BufRead, Result};
use std::path::Path;

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Please provide a file path");

    let res = part_1(file_path).unwrap();
    println!("Result 1: {} ", res);

    let res2 = part_2(file_path).unwrap();
    println!("Result 2: {}", res2);
}

fn part_1<P>(filename: P) -> Result<i32>
where
    P: AsRef<Path>,
{
    Ok(read_lines(filename)?
        .filter_map(Result::ok)
        .map(|line| {
            line.chars()
                .filter(|char| char.is_numeric())
                .map(|char| char.to_digit(16).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|digits| {
            let val = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            val.parse::<i32>().unwrap()
        })
        .sum::<i32>())
}

fn extract_numbers(line: String) -> Vec<u32> {
    use regex::Regex;

    let re = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut acc: Vec<u32> = Vec::new();
    let mut cur = line.clone();

    while !cur.is_empty() {
        if let Some(caps) = re.captures(&cur) {
            let num = match caps.get(1).unwrap().as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            };

            acc.push(num);
        }
        cur.remove(0);
    }

    acc
}

fn part_2<P>(filename: P) -> Result<i32>
where
    P: AsRef<Path>,
{
    Ok(read_lines(filename)?
        .filter_map(Result::ok)
        .map(extract_numbers)
        .map(|digits| {
            let val = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            val.parse::<i32>().unwrap()
        })
        .sum::<i32>())
}

#[test]
fn test_part_1() {
    let res = part_1("./example.txt").unwrap();
    assert_eq!(res, 142);
}

#[test]
fn test_part_2() {
    let res = part_2("./example-2.txt").unwrap();
    assert_eq!(res, 281);
}
