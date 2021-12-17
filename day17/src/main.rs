#[derive(Debug)]
struct Area {
    x: (i32, i32),
    y: (i32, i32),
}

fn parse_pair(s: &str) -> (i32, i32) {
    let v = s[2..]
        .split("..")
        .map(|i| i.trim_end_matches(",").parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    (v[0], v[1])
}

fn read_input() -> Area {
    let v = include_str!("../input")
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    Area {
        x: parse_pair(v[2]),
        y: parse_pair(v[3]),
    }
}

fn eval_trajectory(mut sx: i32, mut sy: i32, a: &Area) -> Option<i32> {
    let dy = sy;
    let mut x = 0;
    let mut y = 0;
    while x <= a.x.1 && (sx > 0 || x >= a.x.0) && (sy >= a.y.0) {
        x += sx;
        y += sy;
        sx = std::cmp::max(0, sx - 1);
        sy -= 1;
        if x >= a.x.0 && x <= a.x.1 && y >= a.y.0 && y <= a.y.1 {
            return Some((dy * dy + dy) / 2);
        }
    }
    None
}

fn part1(a: Area) -> i32 {
    let mut max_y = 0;
    for sx in 0..=a.x.1 {
        for sy in 0..100 {
            if let Some(my) = eval_trajectory(sx, sy, &a) {
                max_y = std::cmp::max(max_y, my);
            }
        }
    }
    max_y
}

fn part2(a: Area) -> i32 {
    let mut c = 0;
    for sx in 0..=a.x.1 {
        for sy in (a.y.0)..100 {
            if let Some(_) = eval_trajectory(sx, sy, &a) {
                c += 1;
            }
        }
    }
    c
}

fn main() {
    println!("{}", part1(read_input()));
    println!("{}", part2(read_input()));
}

#[test]
fn test_example_1() {
    assert_eq!(
        part1(Area {
            x: (20, 30),
            y: (-10, -5)
        }),
        45
    );
}

#[test]
fn test_example_2() {
    assert_eq!(
        part2(Area {
            x: (20, 30),
            y: (-10, -5)
        }),
        112,
    );
}
// Max y: (sy*sy + sy)/2
// sx,            sy
// sx + (sx - 1), sy + sy - 1
//
// Final x = sum 0..sx => (sx*sx + sx)/2
// (sx*sx + sx)/2 = lx
// sx*sx + sx -2lx =
//
// sy
// sy + (sy - 1)
// sy + (sy - 2) + (sy - 3)
// sy + (sy - 1) + (sy - 2) + (sy - 3) + (sy - 4)

// nsy - (n^2-n)/2 = Y
// 2*n*sy - n^2 + n = 2*Y
// 2*n*sy - n^2 + n = 2*Y
// 2*n*sy = (2*Y + n^2 - n)
//
// sy = (2*Y + n^2 - n)/(2*n)
