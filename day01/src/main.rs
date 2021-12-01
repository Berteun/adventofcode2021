fn read_input() -> Vec<i32> {
    return include_str!("../input")
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
}

fn part1(inp: &[i32]) {
    println!("{}", inp.windows(2).filter(|p| p[1] > p[0]).count());
}

fn part2(inp: &[i32]) {
    let result: Vec<i32> = inp.windows(3).map(|s| s.iter().sum()).collect();
    part1(&result)
}

fn main() {
    let inp = read_input();
    part1(&inp);
    part2(&inp);
}
