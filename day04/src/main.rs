#![feature(drain_filter)]

#[derive(Debug)]
struct Card {
    contents: [[i32; 5]; 5],
    crossed: [[bool; 5]; 5],
}

#[derive(Debug)]
struct Input {
    numbers: Vec<i32>,
    cards: Vec<Card>,
}

impl Card {
    fn sum(&self) -> i32 {
        self.crossed
            .iter()
            .flat_map(|r| r.iter())
            .zip(self.contents.iter().flat_map(|r| r.iter()))
            .filter(|(c, _)| !**c)
            .map(|(_, n)| n)
            .sum()
    }

    fn cross_out(&mut self, num: &i32) {
        for y in 0..5 {
            for x in 0..5 {
                if self.contents[y][x] == *num {
                    self.crossed[y][x] = true;
                    return;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        // Horizontal
        self.crossed.iter().any(|r| r.iter().all(|x| *x))
            // Vertical
            || (0..5).any(|n| (0..5).map(|y| self.crossed[y][n]).all(|c| c))
    }
}

fn parse_card(card_str: &str) -> Card {
    let mut card_it = card_str.split_ascii_whitespace();
    let mut card = Card {
        crossed: Default::default(),
        contents: Default::default(),
    };
    for y in 0..5 {
        for x in 0..5 {
            card.contents[y][x] = card_it.next().unwrap().parse::<i32>().unwrap();
        }
    }
    card
}

fn read_input() -> Input {
    let mut inp = include_str!("../input").split("\n\n");
    let numbers: Vec<i32> = inp
        .next()
        .unwrap()
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect();
    let cards: Vec<Card> = inp.map(parse_card).collect();
    Input { numbers, cards }
}

fn part1(mut inp: Input) -> i32 {
    for n in inp.numbers.iter() {
        for c in &mut inp.cards {
            c.cross_out(n);
            if c.has_bingo() {
                return c.sum() * n;
            }
        }
    }
    return 0;
}

fn part2(mut inp: Input) -> i32 {
    let mut last_sum = 0;
    inp.numbers
        .iter()
        .skip_while(|n| {
            inp.cards.drain_filter(|c| {
                c.cross_out(n);
                last_sum = c.sum();
                c.has_bingo()
            });
            inp.cards.len() > 0
        })
        .next()
        .unwrap()
        * last_sum
}

fn main() {
    println!("part1: {}", part1(read_input()));
    println!("part2: {}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 33348);
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 8112);
}
