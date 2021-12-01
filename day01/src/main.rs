fn read_input() -> Vec<i32> {
    return include_str!("../input").lines().map(|i| i.parse::<i32>().unwrap()).collect();
}

fn part1(inp: &[i32]) {
    let mut prev = &i32::MAX;
    let mut cnt = 0;
    for n in inp {
        cnt += (n > &prev) as i32;
        prev = &n;
    }
    println!("{}", cnt)
}

fn part2(inp: &[i32]) {
    let mut end   = 3;
    let mut prev = i32::MAX;
    let mut cnt = 0;
    while end <= inp.len() {
        let sum = inp[(end-3)..end].iter().sum();
        cnt += (sum > prev) as i32;
        prev = sum;
        end+=1;
    }
    println!("{}", cnt)
}

fn main() {
    let inp = read_input();
    part1(&inp);
    part2(&inp);
}
