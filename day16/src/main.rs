fn to_bin(c: char) -> [i64; 4] {
    let v = if c >= '0' && c <= '9' {
        c as i64 - '0' as i64
    } else {
        c as i64 - 'A' as i64 + 10
    };
    [(v & 8) >> 3, (v & 4) >> 2, (v & 2) >> 1, v & 1]
}

struct LitPacket {
    version: i64,
    value: i64,
}

struct OpPacket {
    version: i64,
    ptype: i64,
    subpackets: Vec<Packet>,
}

enum Packet {
    Lit(LitPacket),
    Op(OpPacket),
}

fn read_input() -> Vec<i64> {
    include_str!("../input")
        .chars()
        .filter(|c| *c >= '0' && *c <= 'F')
        .flat_map(|c| to_bin(c))
        .collect::<Vec<_>>()
}

fn read_num<'a>(mut bits: i64, inp: &mut std::slice::Iter<i64>) -> i64 {
    let mut tot = 0;
    while bits > 0 {
        tot = (tot * 2) + inp.next().unwrap();
        bits -= 1;
    }
    tot
}

fn get_version<'a>(inp: &mut std::slice::Iter<i64>) -> i64 {
    read_num(3, inp)
}

fn get_type<'a>(inp: &mut std::slice::Iter<i64>) -> i64 {
    read_num(3, inp)
}

fn parse_subpackets<'a>(mut inp: std::slice::Iter<i64>) -> (std::slice::Iter<i64>, Vec<Packet>) {
    let type_id = *inp.next().unwrap();
    if type_id == 0 {
        let n_bits = read_num(15, &mut inp);
        let input = inp
            .by_ref()
            .take(n_bits as usize)
            .map(|c| *c)
            .collect::<Vec<i64>>();
        let mut res: Vec<Packet> = vec![];
        let mut new_it = input.iter();
        while new_it.len() > 0 {
            let r = parse_packet(new_it);
            new_it = r.0;
            res.push(r.1);
        }
        (inp, res)
    } else {
        let n_packets = read_num(11, &mut inp);
        let mut res: Vec<Packet> = vec![];
        for _ in 0..n_packets {
            let r = parse_packet(inp);
            inp = r.0;
            res.push(r.1);
        }
        (inp, res)
    }
}

fn parse_value<'a>(mut inp: &mut std::slice::Iter<i64>) -> i64 {
    let mut total = 0;
    let mut next = true;
    while next {
        next = *inp.next().unwrap() == 1;
        total = (total * 16) + read_num(4, &mut inp);
    }
    total
}

fn sum_versions(p: &Packet) -> i64 {
    match p {
        Packet::Lit(p) => p.version,
        Packet::Op(p) => p.subpackets.iter().map(|p| sum_versions(p)).sum::<i64>() + p.version,
    }
}

fn evaluate(p: &Packet) -> i64 {
    match p {
        Packet::Lit(p) => p.value,
        Packet::Op(p) => match p.ptype {
            0 => p.subpackets.iter().map(|p| evaluate(p)).sum::<i64>(),
            1 => p.subpackets.iter().fold(1, |acc, p| acc * evaluate(p)),
            2 => p
                .subpackets
                .iter()
                .fold(i64::MAX, |acc, p| std::cmp::min(acc, evaluate(p))),
            3 => p
                .subpackets
                .iter()
                .fold(0, |acc, p| std::cmp::max(acc, evaluate(p))),
            5 => (evaluate(&p.subpackets[0]) > evaluate(&p.subpackets[1])) as i64,
            6 => (evaluate(&p.subpackets[0]) < evaluate(&p.subpackets[1])) as i64,
            7 => (evaluate(&p.subpackets[0]) == evaluate(&p.subpackets[1])) as i64,
            _ => panic!("wrong"),
        },
    }
}

fn parse_packet<'a>(mut it: std::slice::Iter<i64>) -> (std::slice::Iter<i64>, Packet) {
    let version = get_version(&mut it);
    let ptype = get_type(&mut it);
    let p = if ptype == 4 {
        Packet::Lit(LitPacket {
            version,
            value: parse_value(&mut it),
        })
    } else {
        let sp = parse_subpackets(it);
        it = sp.0;
        Packet::Op(OpPacket {
            version,
            ptype,
            subpackets: sp.1,
        })
    };
    (it, p)
}

fn part1(inp: Vec<i64>) -> i64 {
    sum_versions(&parse_packet(inp.iter()).1)
}

fn part2(inp: Vec<i64>) -> i64 {
    evaluate(&parse_packet(inp.iter()).1)
}

fn main() {
    println!("{:?}", part1(read_input()));
    println!("{:?}", part2(read_input()));
}
