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

fn apply_rules(polymer: Vec<char>, rules: &HashMap<String, char>) -> Vec<char> {
  let mut output = vec![];
  output.push(polymer[0]);
  polymer.windows(2).for_each(|w| {
    output.push(*rules.get(&String::from_iter(w.iter())).unwrap());
    output.push(w[1]);
  });
  output
}

fn count(polymer: &Vec<char>) -> i32 {
  let mut counts: HashMap<char, i32> = HashMap::new();
  polymer.iter().for_each(|c| *counts.entry(*c).or_insert(0) += 1);
  let mut v: Vec<i32> = counts.values().map(|i| *i).collect();
  v.sort();
  v[v.len() - 1 as usize] - v[0]
}

fn part_1(inp: (Vec<char>, HashMap<String, char>)) -> i32 {
  let mut polymer = inp.0;
  let table = inp.1;
  for _ in 0..10 {
    polymer = apply_rules(polymer, &table);
  }
  count(&polymer)
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
    //println!("{}", part_2(read_input()));
}
