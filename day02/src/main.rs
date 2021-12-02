enum Direction {
  UP,
  DOWN,
  FORWARD,
}

struct Instruction {
  direction: Direction,
  amount: i32,
}

fn parse_line(line: &str) -> Instruction {
  let instruction: Vec<&str> = line.split_ascii_whitespace().collect();
  let amount: i32 = instruction[1].parse().unwrap();
  match instruction[0] {
    "up" => return Instruction{direction: Direction::UP, amount: amount},
    "down" => return Instruction{direction: Direction::DOWN, amount: amount},
    "forward" => return Instruction{direction: Direction::FORWARD, amount: amount},
    _ => panic!("invalid input")
  }
}

fn read_input() -> Vec<Instruction> {
    return include_str!("../input")
        .lines()
        .map(parse_line)
        .collect();
}

struct Part1Pos {
  x: i32,
  y: i32,
}

fn apply_instruction_1(pos: Part1Pos, instr: &Instruction) -> Part1Pos {
  match instr.direction {
    Direction::UP => return Part1Pos{x: pos.x, y: pos.y - instr.amount},
    Direction::DOWN => return Part1Pos{x: pos.x, y: pos.y + instr.amount},
    Direction::FORWARD => return Part1Pos{x: pos.x + instr.amount, y: pos.y},
  }
}
fn part1(inp: &[Instruction]) {
    let start_position = Part1Pos{x: 0, y: 0};
    let end_position = inp.iter().fold(start_position, apply_instruction_1);
    println!("({},{}): {}", end_position.x, end_position.y, end_position.x * end_position.y);
}

struct Part2Pos {
  x: i32,
  y: i32,
  aim: i32,
}

fn apply_instruction_2(pos: Part2Pos, instr: &Instruction) -> Part2Pos {
  match instr.direction {
    Direction::UP => return Part2Pos{x: pos.x, y: pos.y, aim: pos.aim - instr.amount},
    Direction::DOWN => return Part2Pos{x: pos.x, y: pos.y, aim: pos.aim + instr.amount},
    Direction::FORWARD => return Part2Pos{x: pos.x + instr.amount, y: pos.y + pos.aim * instr.amount, aim: pos.aim},
  }
}

fn part2(inp: &[Instruction]) {
    let start_position = Part2Pos{x: 0, y: 0, aim: 0};
    let end_position = inp.iter().fold(start_position, apply_instruction_2);
    println!("({},{},{}): {}", end_position.x, end_position.y, end_position.aim, end_position.x * end_position.y);
}

fn main() {
    let inp = read_input();
    part1(&inp);
    part2(&inp);
}
