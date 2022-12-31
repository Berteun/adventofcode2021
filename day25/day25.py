#!/usr/bin/env python
# -*- coding: utf-8 -*-
import sys
def read_input(filename):
    return [[c for c in l.strip()] for l in open(filename)]

def print_grid(grid):
    print("\n".join(''.join(row) for row in grid))
    print()

def evolve(grid):
    move_r = set()
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            nb = (x + 1) % len(grid[y])
            if grid[y][x] == '>' and grid[y][nb] == '.': 
                move_r.add(((x,y),(nb,y)))

    for ((x,y),(nx,ny)) in move_r:
        grid[y][x] = '.'
        grid[ny][nx] = '>'

    move_d = set()
    for x in range(len(grid[0])):
        for y in range(len(grid)):
            nb = (y + 1) % len(grid)
            if grid[y][x] == 'v' and grid[nb][x] == '.': 
                move_d.add(((x,y),(x,nb)))

    for ((x,y),(nx,ny)) in move_d:
        grid[y][x] = '.'
        grid[ny][nx] = 'v'

    return len(move_d) + len(move_r)

def part1(grid):
    step = 0
    while True:
        step += 1
        moves = evolve(grid)
        if moves == 0:
            break
    return step


def main(filename):
    grid = read_input(filename) 
    print(part1(grid))

if __name__ == '__main__':
    main('input' if len(sys.argv) == 1 else sys.argv[1])
