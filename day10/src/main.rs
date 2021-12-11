fn value(c: char) -> i64 {
    match c {
        '(' => 3,
        '[' => 57,
        '{' => 1197,
        '<' => 25137,
        _ => panic!("illegal input"),
    }
}

fn check(c1: char, stack: &mut Vec<char>) -> i64 {
    match stack.pop() {
        Some(c2) => {
            if c1 == c2 {
                0
            } else {
                value(c1)
            }
        }
        None => 0,
    }
}

fn score_line_1(s: &String) -> i64 {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        let score = match c {
            '(' | '[' | '<' | '{' => {
                stack.push(c);
                0
            }
            ')' => check('(', &mut stack),
            ']' => check('[', &mut stack),
            '>' => check('<', &mut stack),
            '}' => check('{', &mut stack),
            _ => panic!("illegal input"),
        };
        if score > 0 {
            return score;
        }
    }
    0
}

fn part1(inp: Vec<String>) -> i64 {
    inp.iter().map(score_line_1).sum()
}

fn check_2(c1: char, stack: &mut Vec<char>) -> bool {
    match stack.pop() {
        Some(c2) => c1 != c2,
        None => false,
    }
}

fn score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn score_line_2(s: &String) -> i64 {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        let corrupt = match c {
            '(' | '[' | '<' | '{' => {
                stack.push(c);
                false
            }
            ')' => check_2('(', &mut stack),
            ']' => check_2('[', &mut stack),
            '>' => check_2('<', &mut stack),
            '}' => check_2('{', &mut stack),
            _ => panic!("illegal input"),
        };
        if corrupt {
            return 0;
        }
    }
    stack.iter().rev().fold(0, |acc, c| 5 * acc + score(*c))
}

fn part2(inp: Vec<String>) -> i64 {
    let mut s: Vec<i64> = inp.iter().map(score_line_2).filter(|s| *s > 0).collect();
    s.sort();
    s[s.len() / 2 as usize]
}

fn read_input() -> Vec<String> {
    include_str!("../input")
        .lines()
        .map(|s| String::from(s))
        .collect()
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 167379)
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 2776842859)
}
