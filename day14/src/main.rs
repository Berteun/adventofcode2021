use std::collections::HashMap;

fn parse_polymer(polymer: &str) -> Vec<char> {
  polymer.chars().collect()
}

fn parse_rule(rule: &str) -> (String, char) {
  let mut it = rule.split(" -> ");
  (String::from(it.next().unwrap()), it.next().unwrap().chars().next().unwrap())
}

fn parse_rules(rules: &str) -> HashMap<String, char> {
    rules
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(parse_rule)
        .collect()
}

fn count2(polymer: &Vec<char>, count_map: HashMap<String, i64>) -> i64 {
  let mut counts: HashMap<char, i64> = HashMap::new();
  count_map.iter().for_each(|(s,c)| {
    let chs: Vec<char> = s.chars().collect();
    *counts.entry(chs[0]).or_insert(0) += c;
    *counts.entry(chs[1]).or_insert(0) += c;
  });

  *counts.get_mut(&polymer[0]).unwrap() += 1;
  *counts.get_mut(&polymer[polymer.len() - 1]).unwrap() += 1;

  let mut v: Vec<i64> = counts.values().map(|i| *i).collect();
  v.sort();
  v[v.len() - 1 as usize]/2 - v[0]/2
}

fn apply_rules_2(counts: HashMap<String, i64>, rules: &HashMap<String, char>) -> HashMap<String, i64> {
  let mut deltas: HashMap<String, i64> = HashMap::new();
  for (w, c) in counts.iter() {
    let r = *rules.get(w).unwrap();
    let chs: Vec<char> = w.chars().collect();
    let r1: String = [chs[0], r].iter().collect();
    let r2: String = [r, chs[1]].iter().collect();
    *deltas.entry(r1).or_insert(0) += c;
    *deltas.entry(r2).or_insert(0) += c;
  }   
  deltas
}

fn part_n(inp: (Vec<char>, HashMap<String, char>), iter: i32) -> i64 {
  let mut count_map: HashMap<String, i64> = HashMap::new();
  let table = inp.1;
  inp.0.windows(2).for_each(|w| {
    *count_map.entry(String::from_iter(w.iter())).or_insert(0) += 1;
  });

  for _ in 0..iter {
    count_map = apply_rules_2(count_map, &table);
  }

  count2(&inp.0, count_map)
}

fn part_2(inp: (Vec<char>, HashMap<String, char>)) -> i64 {
  part_n(inp, 40)
}

fn part_1(inp: (Vec<char>, HashMap<String, char>)) -> i64 {
  part_n(inp, 10)
}

fn read_input() -> (Vec<char>, HashMap<String, char>) {
    let mut parts = include_str!("../input").split("\n\n");
    (
        parse_polymer(parts.next().expect("cannot find polymer in input")),
        parse_rules(parts.next().expect("cannot find rules in input")),
    )
}

fn main() {
    println!("{}", part_1(read_input()));
    println!("{}", part_2(read_input()));
}
