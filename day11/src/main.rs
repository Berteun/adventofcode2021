type Board = [[i32; 10]; 10];

fn inc(x: i32, y: i32, board: &mut Board) {
    if x >= 0 && x < 10 && y >= 0 && y < 10 {
        if board[y as usize][x as usize] > 0 {
            board[y as usize][x as usize] += 1;
        }
    }
}

fn inc_neighbours(x: i32, y: i32, board: &mut Board) {
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            if dx == dy && dx == 0 {
                continue;
            }
            inc(x + dx, y + dy, board);
        }
    }
}

fn print_board(inp: &Board) {
    for row in inp {
        println!("{}", row.map(|i| i.to_string()).join(""))
    }
}

fn step(mut inp: &mut Board) -> i32 {
    for y in 0..10 {
        for x in 0..10 {
            inp[y][x] += 1
        }
    }

    let mut flashed = true;
    let mut flashes = 0;
    while flashed {
        flashed = false;
        for y in 0..10 {
            for x in 0..10 {
                if inp[y][x] > 9 {
                    inc_neighbours(x as i32, y as i32, &mut inp);
                    inp[y][x] = 0;
                    flashes += 1;
                    flashed = true;
                }
            }
        }
    }
    flashes
}

fn read_input() -> Board {
    let mut r: Board = Default::default();
    include_str!("../input")
        .lines()
        .enumerate()
        .for_each(|(y, s)| {
            s.chars().enumerate().for_each(|(x, c)| {
                r[y][x] = (c as i32) - ('0' as i32);
            })
        });
    r
}

fn part2(mut inp: Board) -> i32 {
    let mut n = 1;
    loop {
        //println!("step {}", n);
        //print_board(&inp);
        if step(&mut inp) == 100 {
            return n;
        }
        n += 1
    }
}

fn part1(mut inp: Board) -> i32 {
    (0..100).map(|_| step(&mut inp)).sum()
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_part1() {
    assert_eq!(part1(read_input()), 1665)
}

#[test]
fn test_part2() {
    assert_eq!(part2(read_input()), 235)
}
