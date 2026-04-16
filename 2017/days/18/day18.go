package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day18 struct{}

func (d Day18) Part1(input string) string {
	comp := newComputer(input, 0)
	lastSnd := 0
	for {
		inst := comp.program[comp.pc]
		switch inst[0] {
		case "snd":
			lastSnd = comp.getValue(inst[1])
		case "set":
			comp.set(inst[1], comp.getValue(inst[2]))
		case "add":
			comp.set(inst[1], comp.getValue(inst[1])+comp.getValue(inst[2]))
		case "mul":
			comp.set(inst[1], comp.getValue(inst[1])*comp.getValue(inst[2]))
		case "mod":
			comp.set(inst[1], comp.getValue(inst[1])%comp.getValue(inst[2]))
		case "rcv":
			if comp.getValue(inst[1]) != 0 {
				return fmt.Sprintf("%d", lastSnd)
			}
		case "jgz":
			if comp.getValue(inst[1]) > 0 {
				comp.pc += comp.getValue(inst[2]) - 1
			}
		}
		comp.pc++
	}
}

func (d Day18) Part2(input string) string {
	p0 := newComputer(input, 0)
	p1 := newComputer(input, 1)
	q0, q1 := make([]int, 0), make([]int, 0)
	count1 := 0

	for {
		p0.run(true, &q0, &q1)
		p1.run(false, &q1, &q0)

		if p0.blocked && p1.blocked { break }
		for len(q1) > 0 {
			v := q1[0]; q1 = q1[1:]
			p0.q = append(p0.q, v)
			p0.blocked = false
		}
		for len(q0) > 0 {
			v := q0[0]; q0 = q0[1:]
			p1.q = append(p1.q, v)
			p1.blocked = false
			count1++
		}
	}
	return fmt.Sprintf("%d", count1)
}

type Computer struct {
	program [][]string
	pc      int
	regs    map[string]int
	q       []int
	blocked bool
}

func newComputer(input string, id int) *Computer {
	prog := [][]string{}
	for _, line := range strings.Split(input, "\n") {
		if line != "" { prog = append(prog, strings.Fields(line)) }
	}
	return &Computer{program: prog, regs: map[string]int{"p": id}}
}

func (c *Computer) getValue(s string) int {
	if val, err := strconv.Atoi(s); err == nil { return val }
	return c.regs[s]
}

func (c *Computer) set(reg string, val int) { c.regs[reg] = val }

func (c *Computer) run(isP0 bool, in, out *[]int) {
	for c.pc >= 0 && c.pc < len(c.program) {
		inst := c.program[c.pc]
		switch inst[0] {
		case "snd":
			*out = append(*out, c.getValue(inst[1]))
		case "set":
			c.set(inst[1], c.getValue(inst[2]))
		case "add":
			c.set(inst[1], c.getValue(inst[1])+c.getValue(inst[2]))
		case "mul":
			c.set(inst[1], c.getValue(inst[1])*c.getValue(inst[2]))
		case "mod":
			c.set(inst[1], c.getValue(inst[1])%c.getValue(inst[2]))
		case "rcv":
			if len(c.q) == 0 {
				c.blocked = true
				return
			}
			c.set(inst[1], c.q[0]); c.q = c.q[1:]
		case "jgz":
			if c.getValue(inst[1]) > 0 {
				c.pc += c.getValue(inst[2]) - 1
			}
		}
		c.pc++
	}
}

func main() { common.Run(2017, 18, Day18{}) }
