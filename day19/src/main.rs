use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::max;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Scanner {
    coordinates: Vec<Coordinate>
}

#[derive(Debug)]
struct ScannerMap {
    scanners: Vec<Scanner>
}

fn parse_line(l: &str) -> Coordinate {
    let coordinates: Vec<i32> = l.split(",").map(|c| c.parse::<i32>().unwrap()).collect();
    Coordinate{ x: coordinates[0], y: coordinates[1], z: coordinates[2] }
}

fn parse_scanner(l: &str) -> Scanner {
    let mut coordinates : Vec<Coordinate> = l.split("\n").skip(1).filter(|l| l.len() > 0).map(|l| parse_line(l)).collect::<Vec<Coordinate>>();
    coordinates.sort();
    Scanner { coordinates }
}

fn read_input() -> ScannerMap {
    ScannerMap{ scanners: include_str!("../input").split("\n\n").map(|l| parse_scanner(l)).collect() }
}

fn generate_orientations(scan: &Vec<Coordinate>) -> Vec<Vec<Coordinate>> {
    let mut result = Vec::new();
    for _ in 0..24 {
        result.push(Vec::new());
    }

    for coord in scan {
        let m = vec![
          Coordinate { x :  coord.x, y :  coord.y, z :  coord.z,  },
          Coordinate { x :  coord.x, y : -coord.y, z : -coord.z,  },
          Coordinate { x :  coord.x, y :  coord.z, z : -coord.y,  },
          Coordinate { x :  coord.x, y : -coord.z, z :  coord.y,  },
          Coordinate { x : -coord.x, y :  coord.y, z : -coord.z,  },
          Coordinate { x : -coord.x, y : -coord.y, z :  coord.z,  },
          Coordinate { x : -coord.x, y :  coord.z, z :  coord.y,  },
          Coordinate { x : -coord.x, y : -coord.z, z : -coord.y,  },

          Coordinate { x :  coord.y, y :  coord.x, z : -coord.z,  },
          Coordinate { x :  coord.y, y : -coord.x, z :  coord.z,  },
          Coordinate { x :  coord.y, y :  coord.z, z :  coord.x,  },
          Coordinate { x :  coord.y, y : -coord.z, z : -coord.x,  },
          Coordinate { x : -coord.y, y :  coord.x, z :  coord.z,  },
          Coordinate { x : -coord.y, y : -coord.x, z : -coord.z,  },
          Coordinate { x : -coord.y, y :  coord.z, z : -coord.x,  },
          Coordinate { x : -coord.y, y : -coord.z, z :  coord.x,  },

          Coordinate { x :  coord.z, y :  coord.x, z :  coord.y,  },
          Coordinate { x :  coord.z, y : -coord.x, z : -coord.y,  },
          Coordinate { x :  coord.z, y :  coord.y, z : -coord.x,  },
          Coordinate { x :  coord.z, y : -coord.y, z :  coord.x,  },
          Coordinate { x : -coord.z, y :  coord.x, z : -coord.y,  },
          Coordinate { x : -coord.z, y : -coord.x, z :  coord.y,  },
          Coordinate { x : -coord.z, y :  coord.y, z :  coord.x,  },
          Coordinate { x : -coord.z, y : -coord.y, z : -coord.x,  },
        ];

        for i in 0..24 {
            result[i].push(m[i]);
        }
    }

    for i in 0..24 {
        result[i].sort();
    }

    result
}

fn find_translation(orientation: &Vec<Coordinate>, solution: &HashSet<Coordinate>) -> Option<(Vec<Coordinate>, Coordinate)> {
  let mut seen_deltas: HashMap<Coordinate, i32> = HashMap::new();
    for c1 in orientation {
        for c2 in solution {
            let delta = Coordinate{ x: c2.x - c1.x , y: c2.y - c1.y, z: c2.z - c1.z, };
            seen_deltas.insert(delta, 1 + seen_deltas.get(&delta).unwrap_or(&0));
            if seen_deltas[&delta] >= 12 {
                let translated: Vec<_> = orientation.iter().map(|c| Coordinate {x: c.x + delta.x, y: c.y + delta.y, z: c.z + delta.z, }).collect();
                return Some((translated, delta))
            }
        }
    }
  None
}

fn align_with_solution(scan: &Vec<Coordinate>, solution: &HashSet<Coordinate>) -> Option<(Vec<Coordinate>, Coordinate)> {
    for orientation in generate_orientations(scan) {
        if let Some((translation, offset)) = find_translation(&orientation, &solution) {
            return Some((translation, offset))
        }
    }
    None
}

fn part1(mut map: ScannerMap) -> (HashSet<Coordinate>,Vec<Coordinate>) {
    let mut offsets: Vec<Coordinate> = Vec::new();
    let start = map.scanners.remove(0);
    offsets.push(Coordinate{x : 0, y: 0, z: 0});
    let mut aligned: HashSet<_> = start.coordinates.into_iter().collect();
    let mut i = 0;
    while !map.scanners.is_empty() {
        let scanner = &map.scanners[i];
        if let Some((adjusted,offset)) = align_with_solution(&scanner.coordinates, &aligned) {
            aligned.extend(adjusted.into_iter());
            offsets.push(offset);
            map.scanners.remove(i);
            i = 0;
            continue;
        }
        i += 1;
    }
    (aligned,offsets)
}

fn manhattan(c1: &Coordinate, c2: &Coordinate) -> i32 {
  (c2.x - c1.x).abs() + (c2.y - c1.y).abs() + (c2.z - c1.z).abs()
}

fn part2(offsets: &Vec<Coordinate>) -> i32 {
  let mut max_dist = 0;
  for i in 0..offsets.len() {
    for j in (i + 1)..offsets.len() {
      max_dist = max(max_dist, manhattan(&offsets[i], &offsets[j]))
    }
  }    
  max_dist
}

fn main() {
    let map = read_input();
    let (beacons, offsets) = part1(map);
    println!("{}", beacons.len());
    println!("{}", part2(&offsets));
}
