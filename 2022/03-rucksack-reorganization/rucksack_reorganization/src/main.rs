fn main() {
    use std::{env, fs};

    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).unwrap();
    let result = process_groups(contents);

    println!("Hello, world! {result:?}");
}

fn find_replicated_char(right: &str, left: &str) -> Option<char> {
    right.chars().find(|c| left.contains(*c))
}

fn get_char_priority(letter: char) -> Option<i16> {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(letter)
        .map(|f| (f as i16) + 1)
}

fn process(contents: String) -> i64 {
    contents
        .lines()
        .filter(|str| !str.is_empty())
        .map(|str| str.trim())
        .map(|str| str.split_at(str.len() / 2))
        .map(|(right, left)| find_replicated_char(right, left).unwrap())
        .map(|v| get_char_priority(v).unwrap() as i64)
        .sum::<i64>()
}

fn process_groups(contents: String) -> i64 {
    contents
        .lines()
        .filter(|str| !str.is_empty())
        .map(|str| str.trim())
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            group[0]
                .chars()
                .find(|a| group[1].contains(*a) && group[2].contains(*a))
                .unwrap()
        })
        .map(|v| get_char_priority(v).unwrap() as i64)
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    use crate::{find_replicated_char, get_char_priority, process, process_groups};

    #[test]
    fn part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(process(input), 157)
    }

    #[test]
    fn part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(process_groups(input), 70)
    }

    #[test]

    fn test_find_replicated_char() {
        assert_eq!(
            find_replicated_char("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            Some('p')
        );
        assert_eq!(find_replicated_char("vJrwWtwJgWr", "hcsFMMfFFhF"), None)
    }

    #[test]

    fn test_get_char_priority() {
        assert_eq!(get_char_priority('p'), Some(16));
        assert_eq!(get_char_priority('L'), Some(38));
        assert_eq!(get_char_priority('P'), Some(42));
        assert_eq!(get_char_priority('v'), Some(22));
        assert_eq!(get_char_priority('t'), Some(20));
        assert_eq!(get_char_priority('s'), Some(19));
        assert_eq!(get_char_priority('1'), None);
    }
}
