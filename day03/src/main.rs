fn parse_line(line: &str) -> Vec<i32> {
    line.chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            _ => panic!("invalid input!"),
        })
        .collect()
}

fn read_input() -> Vec<Vec<i32>> {
    return include_str!("../input").lines().map(parse_line).collect();
}

fn gamma(inp: &[Vec<i32>]) -> Vec<i32> {
    let l = inp[0].len();
    let mut c = vec![0; l];
    for v in inp {
        for i in 0..l {
            if v[i] == 1 {
                c[i] += 1;
            }
        }
    }

    let mut g = Vec::new();
    let t = inp.len();
    for i in 0..l {
        if c[i] + c[i] < t {
            g.push(0);
        } else {
            g.push(1);
        }
    }
    g
}

fn epsilon(inp: &[Vec<i32>]) -> Vec<i32> {
    let l = inp[0].len();
    let mut c = vec![0; l];
    for v in inp {
        for i in 0..l {
            if v[i] == 1 {
                c[i] += 1;
            }
        }
    }

    let mut e = Vec::new();
    let t = inp.len();
    for i in 0..l {
        if c[i] + c[i] < t {
            e.push(1)
        } else {
            e.push(0)
        }
    }
    e
}

fn part1(inp: &[Vec<i32>]) {
    let l = inp[0].len();
    let mut c = vec![0; l];
    for v in inp {
        for i in 0..l {
            if v[i] == 1 {
                c[i] += 1;
            }
        }
    }

    let mut g = String::from("");
    let mut e = String::from("");
    let t = inp.len();
    for i in 0..l {
        if c[i] + c[i] < t {
            g += "0";
            e += "1";
        } else {
            g += "1";
            e += "0";
        }
    }
    let gamma = isize::from_str_radix(g.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(e.as_str(), 2).unwrap();
    println!(
        "gamma: {} ({}), epsilon: {}, ({}), power: {}",
        g,
        gamma,
        e,
        epsilon,
        gamma * epsilon,
    )
}

fn part2(inp: Vec<Vec<i32>>) {
    let mut glist = inp.clone();
    let mut idx = 0;
    while glist.len() > 1 {
        let gamma = gamma(&glist);
        glist = glist.into_iter().filter(|v| v[idx] == gamma[idx]).collect();
        idx += 1;
    }

    let mut elist = inp.clone();
    let mut idx = 0;
    while elist.len() > 1 {
        let epsilon = epsilon(&elist);
        elist = elist
            .into_iter()
            .filter(|v| v[idx] == epsilon[idx])
            .collect();
        idx += 1;
    }

    let g: String = glist[0].iter().map(|i| i.to_string()).collect::<String>();
    let e: String = elist[0].iter().map(|i| i.to_string()).collect::<String>();

    let o2g = isize::from_str_radix(g.as_str(), 2).unwrap();
    let co2 = isize::from_str_radix(e.as_str(), 2).unwrap();

    println!(
        "o2: {} ({}), co2: {}, ({}), life support: {}",
        g,
        o2g,
        e,
        co2,
        o2g * co2,
    )
}

fn main() {
    let inp = read_input();
    part1(&inp);
    part2(inp);
}
