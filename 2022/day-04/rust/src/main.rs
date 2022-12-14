fn main() {
    use std::{env, fs};

    let file_path = env::args().nth(1).unwrap_or("input.txt".to_string());
    let contents = fs::read_to_string(file_path).unwrap();
    let result = fully_contains_pairs(contents.clone());
    let result_part2 = any_overlaps(contents);

    println!("part-1 result: {result}");
    println!("part-2 result: {result_part2}");
}

type Section = (i64, i64);
type Pairs = (Section, Section);

fn any_overlaps_sections((right, left): Pairs) -> bool {
    right.0 <= left.1 && right.1 >= left.0
}

fn fully_contains_sections((right, left): Pairs) -> bool {
    (right.0 >= left.0 && right.1 <= left.1) || (left.0 >= right.0 && left.1 <= right.1)
}

fn any_overlaps(contents: String) -> i64 {
    parse(contents, any_overlaps_sections)
}

fn fully_contains_pairs(contents: String) -> i64 {
    parse(contents, fully_contains_sections)
}

fn parse(contents: String, f: fn(Pairs) -> bool) -> i64 {
    fn parse_sections(unparsed_section: &str) -> Option<Section> {
        let (start, end) = unparsed_section.split_once('-')?;
        let to_number = |str: &str| str.trim().parse::<i64>().ok();

        Some((to_number(start)?, to_number(end)?))
    }

    fn parse_pairs(pairs: &str) -> Option<Pairs> {
        let (right, left) = pairs.trim().split_once(",")?;

        let right = parse_sections(right)?;
        let left = parse_sections(left)?;

        Some((right, left))
    }

    contents
        .lines()
        .filter(|str| !str.is_empty())
        .map(|str| parse_pairs(str).unwrap())
        .filter(|pairs| f(*pairs))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use crate::{
        any_overlaps, any_overlaps_sections, fully_contains_pairs, fully_contains_sections,
    };

    #[test]
    fn part_1() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        assert_eq!(fully_contains_pairs(input.to_string()), 2);
    }

    #[test]
    fn part_2() {
        let input = "2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8";

        assert_eq!(any_overlaps(input.to_string()), 4);
    }

    #[test]
    fn test_fully_contains_sections() {
        assert_eq!(fully_contains_sections(((2, 4), (6, 8))), false);
        assert_eq!(fully_contains_sections(((2, 3), (4, 5))), false);
        assert_eq!(fully_contains_sections(((5, 7), (7, 9))), false);
        assert_eq!(fully_contains_sections(((2, 8), (3, 7))), true);
        assert_eq!(fully_contains_sections(((6, 6), (4, 6))), true);
        assert_eq!(fully_contains_sections(((2, 6), (4, 8))), false);
    }

    #[test]
    fn test_any_overlaps_sections() {
        assert_eq!(any_overlaps_sections(((2, 4), (6, 8))), false);
        assert_eq!(any_overlaps_sections(((2, 3), (4, 5))), false);
        assert_eq!(any_overlaps_sections(((5, 7), (7, 9))), true);
        assert_eq!(any_overlaps_sections(((2, 8), (3, 7))), true);
        assert_eq!(any_overlaps_sections(((6, 6), (4, 6))), true);
        assert_eq!(any_overlaps_sections(((2, 6), (4, 8))), true);
    }
}
