use std::cmp::max;

type Grid = Vec<Vec<i32>>;
type Folds = Vec<(usize, usize)>;

fn parse_grid(dots: &str) -> Grid {
    let coord_list: Folds = dots
        .split_ascii_whitespace()
        .map(|l| {
            let coords: Vec<usize> = l
                .split(",")
                .map(|i| i.parse::<usize>().expect("expected i32"))
                .collect();
            (coords[0], coords[1])
        })
        .collect();
    let (max_x, max_y) = coord_list
        .iter()
        .fold((0, 0), |(max_x, max_y), (c_x, c_y)| {
            (max(max_x, *c_x), max(max_y, *c_y))
        });

    let mut grid = vec![vec![0; 1 + max_x as usize]; 1 + max_y as usize];
    for (x, y) in coord_list {
        grid[y][x] = 1;
    }
    grid
}

fn parse_fold_line(l: &str) -> (usize, usize) {
    let parts: Vec<&str> = l[11..].split("=").collect();
    if parts[0] == "x" {
        (parts[1].parse::<usize>().unwrap(), 0)
    } else {
        (0, parts[1].parse::<usize>().unwrap())
    }
}

fn parse_folds(folds: &str) -> Vec<(usize, usize)> {
    folds
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(parse_fold_line)
        .collect()
}

fn fold_x(grid: &mut Grid, x: usize) {
    for y in 0..grid.len() {
        for p in (x + 1)..grid[y].len() {
            if grid[y][p] == 1 {
                grid[y][p] = 0;
                grid[y][2 * x - p] = 1;
            }
        }
    }
}

fn fold_y(grid: &mut Grid, y: usize) {
    for x in 0..grid[y].len() {
        for p in (y + 1)..grid.len() {
            if grid[p][x] == 1 {
                grid[p][x] = 0;
                grid[2 * y - p][x] = 1;
            }
        }
    }
}

fn fold(grid: &mut Grid, fold: &(usize, usize)) {
    if fold.0 == 0 {
        fold_y(grid, fold.1)
    } else {
        fold_x(grid, fold.0)
    }
}

fn count_dots(grid: &Grid) -> i32 {
    grid.iter().map(|r| r.iter().sum::<i32>()).sum()
}

fn print_grid(grid: &Grid) {
    for (i, r) in grid.iter().enumerate() {
        println!(
            "{}",
            r[..40]
                .iter()
                .map(|n| if *n == 1 { '#' } else { '.' })
                .collect::<String>()
        );
        if i > 4 {
            break;
        }
    }
    println!("")
}

fn part_1(inp: (Grid, Folds)) -> i32 {
    let mut grid = inp.0;
    let folds = inp.1;
    fold(&mut grid, &folds[0]);
    count_dots(&grid)
}

fn part_2(inp: (Grid, Folds)) {
    let mut grid = inp.0;
    let folds = inp.1;
    for f in folds {
        fold(&mut grid, &f);
    }
    print_grid(&grid);
}

fn read_input() -> (Grid, Folds) {
    let mut parts = include_str!("../input").split("\n\n");
    let dots = parts.next().expect("cannot find coordinates in input");
    let grid = parse_grid(dots);
    let folds = parts.next().expect("cannot find folds in input");
    let fold_list = parse_folds(folds);
    (grid, fold_list)
}

fn main() {
    println!("{}", part_1(read_input()));
    part_2(read_input());
}
