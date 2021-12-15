use std::collections::HashMap;
use std::collections::BinaryHeap;

#[derive(Clone,Copy,Hash)]
struct Node {
  risk: i32,
}

#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq,PartialOrd,Ord)]
struct Point(usize, usize);

#[derive(Debug,Clone,Copy,Eq,PartialEq,PartialOrd,Ord)]
struct QueueElem {
  weight: i32,
  dist: i32,
  point: Point,
}

fn read_input() -> Vec<Vec<Node>> {
  include_str!("../input")
    .lines()
    .filter(|l| l.len() > 0)
    .map(|s| s.chars().map(|i| (Node{ risk: i as i32 - '0' as i32 })).collect())
    .collect()
}

fn enlarge_input(mut inp: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
  let org_w = inp[0].len();
  // Create 4 horizontal copies
  for cp in 1..5 {
    for y in 0..inp.len() {
      for x in 0..org_w {
        let risk = inp[y][x].risk;
        inp[y].push( Node{ risk: if risk + cp < 10 { risk + cp } else { risk + cp - 9 }});
      }
    }
  }
  //
  // Create 4 vertical copies
  let org_h = inp.len();
  for cp in 1..5 {
    for y in 0..org_h {
      inp.push(
        inp[y].iter().map(|n| {
          Node { risk: if n.risk + cp < 10 { n.risk + cp } else { n.risk + cp - 9 }}
        }).collect::<Vec<Node>>()
      );
    }
  }

  inp
}

fn neighbours(ux: usize, uy: usize, inp: &Vec<Vec<Node>>) -> Vec<Point> {
  let mut res = vec![];
  let x = ux as i32;
  let y = uy as i32;
  for xd in [-1, 1] {
    for yd in [-1, 1] {
      if x + xd >= 0 && x + xd < inp[0].len() as i32 {
        res.push(Point((x + xd) as usize, y as usize));
      }
      if y + yd >= 0 && y + yd < inp.len() as i32 {
        res.push(Point(x as usize, (y + yd) as usize));
      }
    }
  }
  res
}

fn solve(inp: Vec<Vec<Node>>) -> i32 {
  let mut distances: HashMap::<Point, i32> = HashMap::new();
  let mut queue: BinaryHeap<QueueElem> = BinaryHeap::new();

  let end = Point(inp[inp.len() - 1].len() - 1, inp.len() - 1);

  distances.insert(Point(0,0), 0);
  queue.push(QueueElem{ weight: 0, dist: 0, point: Point(0, 0)});

  while let Some(QueueElem { dist, point, .. }) = queue.pop() {
    if point == end {
      return dist;
    } 

    if *distances.get(&point).unwrap_or(&i32::MAX) < dist {
      continue;
    }

    for nb in neighbours(point.0, point.1, &inp) {
      let ndist = dist + inp[nb.1][nb.0].risk;
      if ndist < *distances.get(&nb).unwrap_or(&i32::MAX) {
        queue.push(QueueElem{ weight: -ndist, dist: ndist, point: nb });
        distances.insert(nb, ndist);
      }
    } 
  }
  panic!("no shortest path found")
}

#[allow(dead_code)]
fn print_board(inp: &Vec<Vec<Node>>) {
    for row in inp {
        println!("{}", row.iter().map(|n| n.risk.to_string()).collect::<Vec<String>>().join(""))
    }
}

fn part1(inp: Vec<Vec<Node>>) -> i32 {
  solve(inp)
}

fn part2(inp: Vec<Vec<Node>>) -> i32 {
  solve(enlarge_input(inp))
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
  assert_eq!(part1(read_input()), 717)
}

#[test]
fn test_part2() {
  assert_eq!(part2(read_input()), 2993)
}
