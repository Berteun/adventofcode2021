fn triangle_cost(a: i64, b: i64) -> i64 {
    let n = (a - b).abs();
    n * (n + 1) / 2
}

fn solve<C>(crabs: Vec<i64>, cost: C) -> i64
where
    C: Fn(i64, i64) -> i64,
{
    let mn = *crabs.iter().min().unwrap();
    let mx = *crabs.iter().max().unwrap();
    (mn..=mx)
        .map(|a: i64| crabs.iter().map(|x| cost(a, *x)).sum())
        .min()
        .unwrap()
}

fn part1(crabs: Vec<i64>) -> i64 {
    solve(crabs, |a, b| (a - b).abs())
}

fn part2(crabs: Vec<i64>) -> i64 {
    solve(crabs, triangle_cost)
}

fn read_input() -> Vec<i64> {
    include_str!("../input")
        .lines()
        .next()
        .expect("empty file?")
        .split(",")
        .map(|i| i.parse::<i64>().expect("expected integer"))
        .collect()
}

fn main() {
    println!("part1: {}", part1(read_input()));
    println!("part1: {}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 342534);
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 94004208);
}
