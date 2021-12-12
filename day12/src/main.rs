use std::collections::HashMap;

#[derive(Debug, Default)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}

fn count_paths<'a>(
    g: &'a Graph,
    cur: &'a str,
    seen: &mut HashMap<&'a str, i32>,
    max_visits: i32,
) -> i32 {
    let is_lower = |s: &str| s.chars().all(|c| c.is_ascii_lowercase());

    let mut max = max_visits;
    for (key, visits) in seen.iter() {
        if is_lower(key) && *visits == 2 {
            max = 1;
            break;
        }
    }

    if cur == "start" {
        max = 1;
    }

    if is_lower(cur) && *seen.get(cur).unwrap_or(&0) >= max {
        return 0;
    }

    if cur == "end" {
        return 1;
    }

    *seen.entry(cur).or_insert(0) += 1;
    let sum = g
        .edges
        .get(cur)
        .unwrap()
        .iter()
        .map(|n| count_paths(g, n, seen, max_visits))
        .sum();
    *seen.get_mut(cur).unwrap() -= 1;
    sum
}

fn part1(g: Graph) -> i32 {
    count_paths(&g, "start", &mut HashMap::new(), 1)
}

fn part2(g: Graph) -> i32 {
    count_paths(&g, "start", &mut HashMap::new(), 2)
}

fn read_input() -> Graph {
    let mut g: Graph = Default::default();
    include_str!("../input").lines().for_each(|l| {
        let mut i = l.split("-");
        let fr = i.next().expect("edge does not contain from");
        let to = i.next().expect("edge does not contain to");
        g.edges
            .entry(String::from(fr))
            .or_insert(Default::default())
            .push(String::from(to));
        g.edges
            .entry(String::from(to))
            .or_insert(Default::default())
            .push(String::from(fr));
    });
    g
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 3292)
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 89592)
}
