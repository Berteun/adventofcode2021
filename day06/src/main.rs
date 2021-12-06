fn iterate(mut life_times: [i64; 9], n: i64) -> i64 {
    for _ in 0..n {
        life_times.rotate_left(1);
        life_times[6] += life_times[8]
    }
    life_times.iter().sum()
}

fn parse_start(fish: Vec<i64>) -> [i64; 9] {
    let mut life_times: [i64; 9] = Default::default();
    for f in fish {
        life_times[f as usize] += 1
    }
    life_times
}

fn part2(fish: Vec<i64>) -> i64 {
    let life_times = parse_start(fish);
    return iterate(life_times, 256);
}

fn part1(fish: Vec<i64>) -> i64 {
    let life_times = parse_start(fish);
    return iterate(life_times, 80);
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
    assert_eq!(part1(read_input()), 388419);
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 1740449478328);
}
