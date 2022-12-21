#[macro_use]
extern crate prettytable;

use std::{env, error, fs, str::FromStr, thread, time};

fn main() {
    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());

    println!("part_1: {:?}", part_1_from_file_path(file_path));
}

#[derive(Debug)]
struct Procedure {
    count: u32,
    from: u32,
    to: u32,
}

impl FromStr for Procedure {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let count = s
            .chars()
            .take_while(|&c| c != 'f')
            .filter(|c| c.is_digit(10))
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc + x);

        let from = s
            .chars()
            .skip_while(|&c| c != 'f')
            .take_while(|&c| c != 't')
            .filter(|c| c.is_digit(10))
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc + x);

        let to = s
            .chars()
            .skip_while(|&c| c != 't')
            .filter(|c| c.is_digit(10))
            .flat_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc + x);

        Ok(Procedure { count, from, to })
    }
}

#[derive(Clone, Debug)]
struct Creates {
    stack: Vec<Vec<char>>,
}

impl Creates {
    pub fn reorganize(mut self, procedures: Vec<Procedure>) -> String {
        procedures.iter().for_each(|p| {
            println!("procedure: {p:?}");

            for _ in 0..p.count {
                let from_stack = self.stack.get_mut((p.from - 1) as usize).unwrap();

                if let Some(id) = from_stack.pop() {
                    let to_stack = self.stack.get_mut((p.to - 1) as usize).unwrap();

                    to_stack.push(id);
                }
            }

            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);

            self.print();
        });

        self.top()
    }

    pub fn top(self) -> String {
        self.stack.iter().flat_map(|v| v.last()).collect::<String>()
    }

    pub fn print(&self) {
        use prettytable::{Cell, Table};

        let mut table = Table::new();
        let len = self.stack.iter().fold(0, |acc, crates| {
            if crates.len() > acc {
                crates.len()
            } else {
                acc
            }
        });

        for i in 0..len {
            let mut row = row![];
            for creates in self.stack.clone() {
                let id = creates.get(i).unwrap_or(&'-');
                row.add_cell(Cell::new(id.to_string().as_str()));
            }

            table.add_row(row);
        }

        table.printstd()
    }
}

impl FromStr for Creates {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();

        let mut stack: Vec<Vec<char>> = lines
            .next()
            .ok_or("error parsing creates")?
            .split_whitespace()
            .map(|_| Vec::new())
            .collect();

        lines.for_each(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(idx, c)| {
                    if c.is_alphabetic() {
                        stack[idx].push(c);
                    }
                })
        });

        Ok(Self { stack })
    }
}

fn process_part_1(contents: String) -> Result<String, &'static str> {
    let (crates_drawing, procedures_drawing) =
        contents.split_once("\n\n").ok_or("Error parsing drawing")?;

    let procedures: Vec<Procedure> = procedures_drawing
        .lines()
        .flat_map(|line| line.parse())
        .collect();

    let creates = crates_drawing.parse::<Creates>().unwrap();

    Ok(creates.reorganize(procedures))
}

fn part_1_from_file_path(file_path: String) -> Result<String, &'static str> {
    process_part_1(fs::read_to_string(file_path).map_err(|_path| "a")?)
}

#[cfg(test)]
mod test {
    use crate::part_1_from_file_path;

    #[test]
    fn test_example() {
        assert_eq!(
            part_1_from_file_path("./example.txt".to_string()),
            Ok("CMZ".to_string())
        )
    }
}
