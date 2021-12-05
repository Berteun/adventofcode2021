#[derive(Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Pair {
    fr: Point,
    to: Point,
}

impl Pair {
    fn outer(&self) -> Point {
        return Point(
            std::cmp::max(self.fr.0, self.to.0),
            std::cmp::max(self.to.1, self.to.1),
        );
    }
}

fn parse_line(line: &str) -> Pair {
    let mut parts = line.split(" -> ");
    let mut fr = parts
        .next()
        .expect("cannot parse left side of pair")
        .split(",")
        .map(|p| p.parse::<i32>().expect("coordinate is not an int"));
    let mut to = parts
        .next()
        .expect("cannot parse right side of pair")
        .split(",")
        .map(|p| p.parse::<i32>().expect("coordinate is not an int"));

    return Pair {
        fr: Point(fr.next().unwrap(), fr.next().unwrap()),
        to: Point(to.next().unwrap(), to.next().unwrap()),
    };
}

fn max(inp: &Vec<Pair>) -> Point {
    inp.iter().fold(Point(0, 0), |max, p| {
        Point(
            std::cmp::max(max.0, p.outer().0),
            std::cmp::max(max.1, p.outer().1),
        )
    })
}

fn make_grid(max_coords: Point) -> Vec<Vec<i32>> {
    vec![vec![0; (max_coords.1 * 2) as usize]; (max_coords.0 * 2) as usize]
}

fn count_overlaps(grid: &Vec<Vec<i32>>) -> usize {
    grid.iter()
        .map(|r| r.iter().filter(|p| **p > 1).count())
        .into_iter()
        .sum()
}

fn cross_out_h(grid: &mut Vec<Vec<i32>>, pair: &Pair) {
    let y = pair.fr.1;
    let fr = std::cmp::min(pair.fr.0, pair.to.0);
    let to = std::cmp::max(pair.fr.0, pair.to.0);
    for x in fr..=to {
        grid[y as usize][x as usize] += 1
    }
}
fn cross_out_v(grid: &mut Vec<Vec<i32>>, pair: &Pair) {
    let x = pair.fr.0;
    let fr = std::cmp::min(pair.fr.1, pair.to.1);
    let to = std::cmp::max(pair.fr.1, pair.to.1);
    for y in fr..=to {
        grid[y as usize][x as usize] += 1
    }
}

fn cross_out_d(grid: &mut Vec<Vec<i32>>, pair: &Pair) {
    let (fr, to) = if pair.fr.0 < pair.to.0 {
        (&pair.fr, &pair.to)
    } else {
        (&pair.to, &pair.fr)
    };

    let mut y = fr.1;
    let y_d = if fr.1 < to.1 { 1 } else { -1 };
    for x in fr.0..=to.0 {
        grid[y as usize][x as usize] += 1;
        y += y_d
    }
}

fn cross_out(grid: &mut Vec<Vec<i32>>, pair: &Pair) {
    if pair.fr.0 == pair.to.0 {
        cross_out_v(grid, pair)
    } else if pair.fr.1 == pair.to.1 {
        cross_out_h(grid, pair)
    } else {
        cross_out_d(grid, pair)
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<i32>>) {
    for y in 0..grid.len() {
        println!(
            "{}",
            grid[y]
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
    println!("\n")
}

fn part1(inp: Vec<Pair>) -> usize {
    let mut grid = make_grid(max(&inp));
    for pair in inp {
        cross_out(&mut grid, &pair);
    }
    count_overlaps(&grid)
}

fn part2(inp: Vec<Pair>) -> usize {
    let mut grid = make_grid(max(&inp));
    for pair in inp {
        cross_out(&mut grid, &pair);
        //print_grid(&grid)
    }
    count_overlaps(&grid)
}

fn horiz_or_vert(pair: &Pair) -> bool {
    pair.fr.0 == pair.to.0 || pair.fr.1 == pair.to.1
}

fn read_input() -> Vec<Pair> {
    include_str!("../input")
        .lines()
        .map(parse_line)
        .filter(horiz_or_vert)
        .collect()
}

fn read_input_full() -> Vec<Pair> {
    include_str!("../input").lines().map(parse_line).collect()
}

fn main() {
    println!("part1: {}", part1(read_input()));
    println!("part1: {}", part2(read_input_full()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 5373);
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input_full()), 21514);
}
