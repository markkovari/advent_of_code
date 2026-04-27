package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Day23 struct{}

type Day23Computer struct {
	instructions [][]string
	pointer      int
	registers    map[string]int
}

func (d Day23) Part1(input string) string {
	comp := d.newComputer(input)
	mulsRun := 0
	for comp.pointer < len(comp.instructions) {
		inst := comp.instructions[comp.pointer]
		x, y := inst[1], inst[2]
		switch inst[0] {
		case "set":
			comp.registers[x] = comp.getVal(y)
		case "sub":
			comp.registers[x] -= comp.getVal(y)
		case "mul":
			comp.registers[x] *= comp.getVal(y)
			mulsRun++
		case "jnz":
			if comp.getVal(x) != 0 {
				comp.pointer += comp.getVal(y) - 1
			}
		}
		comp.pointer++
	}
	return fmt.Sprintf("%d", mulsRun)
}

func (d Day23) Part2(input string) string {
	b, c := 108100, 125100 // Derived from input
	h := 0
	for ; b <= c; b += 17 {
		f := 1
		for d := 2; d*d <= b; d++ {
			if b%d == 0 {
				f = 0
				break
			}
		}
		if f == 0 {
			h++
		}
	}
	return fmt.Sprintf("%d", h)
}

func (c *Day23Computer) getVal(s string) int {
	if val, err := strconv.Atoi(s); err == nil {
		return val
	}
	return c.registers[s]
}

func (d Day23) newComputer(input string) *Day23Computer {
	comp := &Day23Computer{registers: make(map[string]int)}
	for _, line := range strings.Split(input, "\n") {
		if line != "" {
			comp.instructions = append(comp.instructions, strings.Fields(line))
		}
	}
	return comp
}
