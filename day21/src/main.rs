use std::collections::HashMap;

#[derive(Debug,Eq,PartialEq,Hash,Clone,Copy)]
struct PlayerState {
  score: usize,
  position: usize,
}

#[derive(Debug)]
struct PlayerStates {
  state: HashMap<(PlayerState,PlayerState), usize>
}

#[derive(Debug)]
struct Die {
  state: usize,
  rolls: usize,
}

impl PlayerStates {
  fn new(start_p1: PlayerState, start_p2: PlayerState) -> PlayerStates {
    let mut state = HashMap::new();
    state.insert((start_p1, start_p2), 1);
    PlayerStates{ state }
  }

  fn split1(&mut self) -> usize {
    let mut new_state = HashMap::new();
    let mut wins = 0;
    for ((p1_pos, p2_pos), cnt) in self.state.iter() {
      for new_pos in p1_pos.split() {
        if new_pos.score < 21 {
          let cur_count = *new_state.get(&(new_pos, *p2_pos)).unwrap_or(&0);
          new_state.insert((new_pos, *p2_pos), cur_count + cnt);
        } else {
          wins += cnt
        }
      }
    }
    self.state = new_state;
    wins
  }

  fn split2(&mut self) -> usize {
    let mut new_state = HashMap::new();
    let mut wins = 0;
    for ((p1_pos, p2_pos), cnt) in self.state.iter() {
      for new_pos in p2_pos.split() {
        if new_pos.score < 21 {
          let cur_count = *new_state.get(&(*p1_pos, new_pos)).unwrap_or(&0);
          new_state.insert((*p1_pos, new_pos), cur_count + cnt);
        } else {
          wins += cnt
        }
      }
    }
    self.state = new_state;
    wins
  }
}

impl PlayerState {
  fn advance(&mut self, spaces: usize) {
    self.position = (self.position + spaces) % 10;
    self.score += self.position + 1;
  }

  fn split(&self) -> [PlayerState; 27] {
      (1..=3).flat_map(|a| (1..=3).flat_map(move |b| (1..=3).map(move |c| a + b + c))).map(|t| PlayerState{ position: (self.position + t) % 10, score: self.score + 1 + ((self.position + t) % 10) }).collect::<Vec<PlayerState>>().try_into().unwrap()
  }
}

impl Die {
  fn new() -> Die {
    Die{ state: 0, rolls: 0 }
  }

  fn roll(&mut self) -> usize {
    let result = self.state + 1;
    self.rolls += 1;
    self.state = result % 100;
    result
  }

  fn roll_3(&mut self) -> usize {
    self.roll() + self.roll() + self.roll()
  }
}

fn read_input() -> (PlayerState,PlayerState) {
  let mut lines = include_str!("../input").lines();
  let p1_str = lines.next().unwrap();
  let p2_str = lines.next().unwrap();
  let (_, p1_pos) = p1_str.split_once(": ").unwrap();
  let (_, p2_pos) = p2_str.split_once(": ").unwrap();
  (PlayerState{
    score: 0,
    position: p1_pos.parse::<usize>().unwrap() - 1,
  },
  PlayerState{
    score: 0,
    position: p2_pos.parse::<usize>().unwrap() - 1,
  })
}

fn part1(mut p1: PlayerState, mut p2: PlayerState) -> usize {
  let mut die = Die::new();
  loop {
    p1.advance(die.roll_3());
    if p1.score >= 1000 {
      return die.rolls * p2.score;
    }
    p2.advance(die.roll_3());
    if p2.score >= 1000 {
      return die.rolls * p1.score;
    }
  }
}

fn part2(p1: PlayerState, p2: PlayerState) -> usize {
  let mut states = PlayerStates::new(p1, p2);
  let mut p1_wins = 0;
  let mut p2_wins = 0;
  while dbg!(states.state.len()) > 0 {
    //dbg!(&states);
    p1_wins += states.split1();
    //dbg!(&states);
    p2_wins += states.split2();
  }
  std::cmp::max(p1_wins, p2_wins)
}

fn main() {
    let (p1, p2) = read_input();
    println!("{}", part1(p1, p2));
    let (p1, p2) = read_input();
    println!("{}", part2(p1, p2));
}

