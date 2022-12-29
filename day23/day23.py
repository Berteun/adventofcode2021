#!/usr/bin/env python
# -*- coding: utf-8 -*-
import heapq
import sys
from collections import defaultdict

def print_map(m):
    for y in range(7):
        for x in range(13):
            c = (x, y)
            if c in m:
                sys.stdout.write(m[c])
            else:
                sys.stdout.write(' ')
        sys.stdout.write("\n")
    sys.stdout.write("\n")

def read_input(filename, insert):
    lines = open(filename).readlines()

    if insert:
        lines = lines[:3] + ["  #D#C#B#A#\n", "  #D#B#A#C#\n"] + lines[3:]
    m = {}
    for y, l in enumerate(lines):
        for x, c in enumerate(l):
            coor = (x, y)
            if c in 'ABCD#.':
                m[coor] = c
    return m

cost_map = {
    'A': 1,
    'B': 10,
    'C': 100,
    'D': 1000,
}

def h(m, cost):
    s = cost
    for (current_loc, animal) in m:
        if animal == 'A':
            s += 1 * int(abs(current_loc[0] - 3) + abs(current_loc[1] - 2))
        elif animal == 'B':
            s += 10 * int(abs(current_loc[0] - 5) + abs(current_loc[1] - 2))
        elif animal == 'C':
            s += 100 * int(abs(current_loc[0] - 7) + abs(current_loc[1] - 2))
        elif animal == 'D':
            s += 1000 * int(abs(current_loc[0] - 9) + abs(current_loc[1] - 2))
    return s

def moves(coord, m):
    seen = { coord: 0 }
    queue = [(coord, 0)]
    while queue:
        c, steps = queue.pop(0)
        for d in ((-1, 0), (1, 0), (0, 1), (0, -1)):
            nb = (c[0] + d[0], c[1] + d[1])
            if m[nb] == '.' and nb not in seen:
                seen[nb] = steps + 1
                queue.append((nb, steps + 1))
    return seen
                
home_cols = {
    'A': 3,
    'B': 5,
    'C': 7,
    'D': 9,
}

def neighbours(m):
    for coord, val in m.items():
        if val in 'ABCD':
            moveset = moves(coord, m)
            for move in moveset:
                x, y = move
                # Can't stop in front of room
                if y == 1 and (x == 3 or x == 5 or x == 7 or x == 9):
                    continue

                # Can't stay in hallway
                if coord[1] == 1 and y == 1:
                    continue

                # Can't move into the wrong column
                if y in (2,3,4,5) and home_cols[val] != x:
                    continue

                # Can't move into the home column when another type of pod is there
                if x == home_cols[val]:
                    column = set(m.get((x,y), '#') for y in range(y + 1,6))
                    column -= {'#', val}
                    if column:
                        continue


                # Correct spot, don't move away
                if coord[0] == home_cols[val] and coord[1] == 3:
                    continue
                cp = m.copy()
                cp[coord] = '.'
                cp[move] = val
                yield tuple(sorted(cp.items())), moveset[move] * cost_map[val] 

def is_end(nb):
    m = dict(nb)
    for ((x,y), v) in m.items():    
        if v in 'ABCD' and not y in (2,3,4,5):
            return False
        if v == 'A' and x != 3:
            return False
        elif v == 'B' and x != 5:
            return False
        elif v == 'C' and x != 7:
            return False
        elif v == 'D' and x != 9:
            return False
    return True


def part12(config):
    start = tuple(sorted(config.items()))

    dist = { start: 0 }
    prev = {}

    # Heurstic score, cost, map
    Q = [ (h(config, 0), 0, start) ]
    while Q:
        score, cost, tm = heapq.heappop(Q)
        m  = dict(tm)
        key = tm
        if key in dist and cost > dist[key]:
            continue

        for nb_map, travel_cost in neighbours(m):
            new_cost = dist[key] + travel_cost
            if is_end(nb_map):
                return new_cost
            if nb_map not in dist or new_cost < dist[nb_map]:
                dist[nb_map] = new_cost
                prev[nb_map] = tm
                heapq.heappush(Q, (h(nb_map, new_cost), new_cost, nb_map))

def main(filename):
    config = read_input(filename, False)
    print(part12(config))

    config = read_input(filename, True)
    print(part12(config))

if __name__ == '__main__':
    main('input' if len(sys.argv) == 1 else sys.argv[1])
