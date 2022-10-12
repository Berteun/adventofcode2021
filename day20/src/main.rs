use std::collections::HashMap;

type Image = HashMap<(i32, i32), bool>;
type Algo = Vec<bool>;

#[derive(Debug,Clone)]
struct Problem {
  algo: Algo,
  image: Image,
  minx: i32,
  maxx: i32,
  miny: i32,
  maxy: i32,
}

fn parse_algorithm(line: &str) -> Algo {
  line.chars().map(|c| c == '#').collect()
}

fn parse_image_line(y: i32, l: &str) -> impl Iterator<Item=((i32, i32), bool)> + '_ {
  l.chars().enumerate().map(move |(x,c)| ((x as i32, y), c == '#'))
}

fn parse_image(lines: &str) -> Image {
  lines.lines().enumerate().map(|(y,l)| parse_image_line(y as i32, l)).flatten().collect()
}

fn read_input() -> Problem {
  let mut inp = include_str!("../input").split("\n\n");
  let algo = parse_algorithm(inp.next().unwrap());
  let image = parse_image(inp.next().unwrap());
  let maxx = *image.iter().map(|((x,_),_)| x).max().unwrap();
  let maxy = *image.iter().map(|((_,y),_)| y).max().unwrap();
  Problem{ algo, image, minx: 0, miny: 0, maxx, maxy }
}

fn compute_index(coord: (i32, i32), problem: &Problem, default: bool) -> usize {
  let (x, y) = coord;  
  256 * *problem.image.get(&(x - 1, y - 1)).unwrap_or(&default) as usize +
  128 * *problem.image.get(&(x    , y - 1)).unwrap_or(&default) as usize +
   64 * *problem.image.get(&(x + 1, y - 1)).unwrap_or(&default) as usize +
   32 * *problem.image.get(&(x - 1, y    )).unwrap_or(&default) as usize +
   16 * *problem.image.get(&(x    , y    )).unwrap_or(&default) as usize +
    8 * *problem.image.get(&(x + 1, y    )).unwrap_or(&default) as usize +
    4 * *problem.image.get(&(x - 1, y + 1)).unwrap_or(&default) as usize +
    2 * *problem.image.get(&(x    , y + 1)).unwrap_or(&default) as usize +
    1 * *problem.image.get(&(x + 1, y + 1)).unwrap_or(&default) as usize 
}

fn iterate(problem: &Problem, default: bool) -> Problem {
  let mut result = Problem { 
    algo: problem.algo.clone(),
    image: HashMap::new(), 
    minx: problem.minx - 2,
    miny: problem.miny - 2,
    maxx: problem.maxx + 2, 
    maxy: problem.maxy + 2 };

  for y in result.miny..=result.maxy {
    for x in result.minx..=result.maxx {
      let coord = (x, y);
      let index = compute_index(coord, problem, default);
      result.image.insert(coord, problem.algo[index]);
    }
  }

  result
}

fn print_problem(problem: &Problem) {
  for y in problem.miny..=problem.maxy {
    for x in problem.minx..=problem.maxx {
      let coord = (x, y);
      print!("{}", match problem.image[&coord] {
        false => '.',
        true => '#'
      });
    }
    print!("\n");
  }
}

fn part1(mut problem: Problem) -> i32 {
  for it in 0..2 {
    problem = iterate(&problem, it % 2 == 1);
  }
  problem.image.iter().filter(|(_, state)| **state ).count() as i32
}

fn part2(mut problem: Problem) -> i32 {
  for it in 0..50 {
    problem = iterate(&problem, it % 2 == 1);
  }
  problem.image.iter().filter(|(_, state)| **state ).count() as i32
}

fn main() {
    let problem = read_input();
    println!("{:?}", part1(problem.clone()));
    println!("{:?}", part2(problem));
}
