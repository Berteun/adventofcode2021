use std::cmp::max;
use std::cmp::min;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LightState {
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cuboid {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Toggle {
    state: LightState,
    cuboid: Cuboid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WCuboid {
    weight: i64,
    cuboid: Cuboid,
}

impl Cuboid {
    fn intersection(&self, cuboid: &Cuboid) -> Option<Cuboid> {
        if self.max_x < cuboid.min_x || self.min_x > cuboid.max_x {
            return None;
        }
        if self.max_y < cuboid.min_y || self.min_y > cuboid.max_y {
            return None;
        }
        if self.max_z < cuboid.min_z || self.min_z > cuboid.max_z {
            return None;
        }

        let x_start = max(self.min_x, cuboid.min_x);
        let x_end = min(self.max_x, cuboid.max_x);

        let y_start = max(self.min_y, cuboid.min_y);
        let y_end = min(self.max_y, cuboid.max_y);

        let z_start = max(self.min_z, cuboid.min_z);
        let z_end = min(self.max_z, cuboid.max_z);

        Some(Cuboid {
            min_x: x_start,
            max_x: x_end,
            min_y: y_start,
            max_y: y_end,
            min_z: z_start,
            max_z: z_end,
        })
    }

    fn size(&self) -> i64 {
        (1 + self.max_x - self.min_x)
            * (1 + self.max_y - self.min_y)
            * (1 + self.max_z - self.min_z)
    }
}

impl WCuboid {
    fn apply(&self, toggle: &Toggle) -> Option<WCuboid> {
        if let Some(intersection) = self.cuboid.intersection(&toggle.cuboid) {
            Some(WCuboid {
                weight: -self.weight,
                cuboid: intersection,
            })
        } else {
            None
        }
    }

    fn value(&self) -> i64 {
        self.cuboid.size() * self.weight
    }
}

fn parse_line(line: &str) -> Toggle {
    let (state_str, cuboid_str) = line.split_once(" ").unwrap();
    let coord_strs: Vec<&str> = cuboid_str.splitn(3, ",").collect();
    let coords = coord_strs
        .iter()
        .map(|s| {
            let (from_str, to_str) = s.split_once("..").unwrap();
            let from = from_str[2..].parse::<i64>().unwrap();
            let to = to_str.parse::<i64>().unwrap();
            (from, to)
        })
        .collect::<Vec<(i64, i64)>>();
    Toggle {
        state: match state_str {
            "on" => LightState::On,
            "off" => LightState::Off,
            _ => panic!("unknown state"),
        },
        cuboid: Cuboid {
            min_x: coords[0].0,
            max_x: coords[0].1,
            min_y: coords[1].0,
            max_y: coords[1].1,
            min_z: coords[2].0,
            max_z: coords[2].1,
        },
    }
}

fn read_input() -> Vec<Toggle> {
    include_str!("../input").lines().map(parse_line).collect()
}

fn part1(toggles: &Vec<Toggle>) -> i64 {
    let mut lights: [[[LightState; 50 + 50 + 1]; 50 + 50 + 1]; 50 + 50 + 1] =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    for z in 0..101 {
        for y in 0..101 {
            for x in 0..101 {
                lights[z][y][x] = LightState::Off;
            }
        }
    }

    let core = Cuboid {
        min_x: -50,
        max_x: 50,
        min_y: -50,
        max_y: 50,
        min_z: -50,
        max_z: 50,
    };

    for toggle in toggles {
        if let Some(intersection) = core.intersection(&toggle.cuboid) {
            for z in intersection.min_z..=intersection.max_z {
                for y in intersection.min_y..=intersection.max_y {
                    for x in intersection.min_x..=intersection.max_x {
                        lights[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] =
                            toggle.state;
                    }
                }
            }
        }
    }

    lights
        .iter()
        .flat_map(|b| b.iter().flat_map(|r| r.iter()))
        .filter(|l| **l == LightState::On)
        .count() as i64
}

fn part2(toggles: &Vec<Toggle>) -> i64 {
    let mut cores: Vec<WCuboid> = Vec::new();

    for toggle in toggles {
        let mut new_cores = Vec::new();
        if toggle.state == LightState::On {
            new_cores.push(WCuboid {
                weight: 1,
                cuboid: toggle.cuboid,
            });
        }
        for core in &cores {
            if let Some(intersection) = core.apply(&toggle) {
                new_cores.push(intersection)
            }
        }
        cores.extend(new_cores);
    }

    cores.iter().map(|c| c.value()).sum()
}

fn main() {
    let cuboids = read_input();
    print!("{}\n", part1(&cuboids));
    print!("{}\n", part2(&cuboids));
}
