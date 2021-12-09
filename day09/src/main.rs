#[derive(Clone,Copy)]
struct Point {
  height: i32,
  basin: i32,
}

fn read_input() -> Vec<Vec<Point>> {
  include_str!("../input")
    .lines()
    .map(|s| s.chars().map(|i| (Point{ height: i as i32 - '0' as i32, basin: -1 })).collect())
    .collect()
}

fn neighbour_coords(ux: usize, uy: usize, inp: &Vec<Vec<Point>>) -> Vec<(usize,usize)> {
  let mut res = vec![];
  let x = ux as i32;
  let y = uy as i32;
  for xd in [-1, 1] {
    for yd in [-1, 1] {
      if x + xd >= 0 && x + xd < inp[0].len() as i32 {
        res.push(((x + xd) as usize, y as usize));
      }
      if y + yd >= 0 && y + yd < inp.len() as i32 {
        res.push((x as usize, (y + yd) as usize));
      }
    }
  }
  res
}
fn neighbours(x: usize, y: usize, inp: &Vec<Vec<Point>>) -> Vec<Point> {
  neighbour_coords(x, y, inp).iter().map(|(x,y)| inp[*y][*x]).collect()
}

fn part1(inp: Vec<Vec<Point>>) -> i32 {
  let mut risk_level = 0;
  for y in 0..inp.len() {
    for x in 0..inp[y].len() {
      if neighbours(x, y, &inp).iter().all(|n| n.height > inp[y][x].height) {
        risk_level += inp[y][x].height + 1;
      }
    }
  }     
  risk_level
}

fn fill_basin(basin_no: i32, x: usize, y: usize, inp: &mut Vec<Vec<Point>>) -> i32 {
  if inp[y][x].height == 9 || inp[y][x].basin > -1 {
    0
  } else {
    inp[y][x].basin = basin_no;
    1 + neighbour_coords(x, y, &inp).iter().map(|n| fill_basin(basin_no, n.0 as usize, n.1 as usize, inp)).sum::<i32>()
  }
}

fn part2(mut inp: Vec<Vec<Point>>) -> i32 {
  let mut basins: Vec<i32> = vec![];
  for y in 0..inp.len() {
    for x in 0..inp[y].len() {
      if inp[y][x].height < 9 && inp[y][x].basin == -1 {
        basins.push(fill_basin(basins.len() as i32, x, y, &mut inp));
      }
    }
  }     
  basins.sort();
  basins.iter().rev().take(3).fold(1, |a,n| a * n)
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
  assert_eq!(part1(read_input()), 631)
}

#[test]
fn test_part2() {
  assert_eq!(part2(read_input()), 821560)
}
