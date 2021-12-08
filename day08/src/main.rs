use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    patterns: Vec<String>,
    output: Vec<String>,
}

static DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn get_digits_map(per: Vec<char>) -> HashMap<String, i32> {
    let mut h = HashMap::new();
    for (i, d) in DIGITS.iter().enumerate() {
        let mut l: Vec<char> = d.chars().map(|c| per[c as usize - 'a' as usize]).collect();
        l.sort_unstable();
        h.insert(l.into_iter().collect(), i as i32);
    }
    h
}

fn sorted(s: &str) -> String {
    let mut l: Vec<char> = s.chars().collect();
    l.sort_unstable();
    l.into_iter().collect()
}

fn parse_line(line: &str) -> Line {
    let mut split = line.split(" | ");
    let pattern_str = split.next().expect("no patterns part found");
    let output_str = split.next().expect("no signals part found");
    return Line {
        patterns: pattern_str
            .trim()
            .split_ascii_whitespace()
            .map(|s| sorted(s))
            .collect(),
        output: output_str
            .trim()
            .split_ascii_whitespace()
            .map(|s| sorted(s))
            .collect(),
    };
}

fn read_input() -> Vec<Line> {
    include_str!("../input").lines().map(parse_line).collect()
}

fn part1(inp: Vec<Line>) -> i32 {
    inp.iter()
        .map(|l| {
            l.output
                .iter()
                .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
                .count()
        })
        .sum::<usize>() as i32
}

fn make_digit(map: &HashMap<String, i32>, output: &Vec<String>) -> i32 {
    output.iter().fold(0, |v, s| (10 * v) + map.get(s).unwrap())
}

fn works(map: &HashMap<String, i32>, signals: &Vec<String>) -> bool {
    signals.iter().all(|s| map.contains_key(s))
}

fn solve_line(line: &Line) -> i32 {
    let all_permutations = || ('a'..='g').into_iter().permutations(7);

    for per in all_permutations() {
        let candidate = get_digits_map(per);
        if works(&candidate, &line.patterns) {
            return make_digit(&candidate, &line.output);
        }
    }
    0
}

fn part2(inp: Vec<Line>) -> i32 {
    inp.iter().map(|m| solve_line(m)).sum()
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 521);
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 1016804);
}
