#!/usr/bin/env python
# -*- coding: utf-8 -*-
import operator
import sys
import z3
from dataclasses import dataclass

@dataclass
class Instr:
    name: str
    args: tuple

def	read_input(filename):
    instrs = []
    for line in open(filename):
        t = line.strip().split(' ')
        instrs.append(Instr(t[0], t[1:]))
    return instrs

def run_program(instrs, solver):
    zero, one = z3.BitVecVal(0, 48), z3.BitVecVal(1, 48)

    inputs = [z3.BitVec(f'inp_{i}', 48) for i in range(14)]
    for inp in inputs:
        solver.add(z3.And(1 <= inp, inp <= 9))

    regs = {r: zero for r in 'wxyz'}

    inp_count = 0
    for ix, i in enumerate(instrs):
        reg = i.args[0]			
        if i.name == 'inp':
            regs[reg] = inputs[inp_count]
            inp_count += 1
        else:
            a = regs[reg]
            try:
                b = int(i.args[1])
            except:
                b = regs[i.args[1]]
            result_ix = z3.BitVec(f'result_{ix}', 48)
            if i.name == 'add':
                solver.add(result_ix == a + b)
            if i.name == 'mul':
                solver.add(result_ix == a * b)
            if i.name == 'div':
                solver.add(result_ix == a / b)
            if i.name == 'mod':
                solver.add(result_ix == a % b)
            if i.name == 'eql':
                solver.add(result_ix == z3.If(a == b, one, zero))
            regs[reg] = result_ix
    solver.add(regs['z'] == 0)
    number = z3.BitVec(f"number", 48)
    solver.add(number == sum((10 ** i) * d for i, d in enumerate(reversed(inputs))))
    return number

def part1(instrs):
    solver = z3.Optimize()
    number = run_program(instrs, solver)

    solver.maximize(number)
    assert solver.check() == z3.sat
    return solver.model().eval(number)

def part2(instrs):
    solver = z3.Optimize()
    number = run_program(instrs, solver)

    solver.minimize(number)
    assert solver.check() == z3.sat
    return solver.model().eval(number)

def main(filename):
    instrs = read_input(filename) 
    print(part1(instrs))
    print(part2(instrs))

if __name__ == '__main__':
    main('input' if len(sys.argv) == 1 else sys.argv[1])
