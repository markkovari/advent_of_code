package day18

import (
	"fmt"
	"strconv"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	comp := d.newComputer(input, 0)
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

func (d Solver) Part2(input string) string {
	p0 := d.newComputer(input, 0)
	p1 := d.newComputer(input, 1)
	count1 := 0

	for {
		p0.run(&p1.q, nil)
		p1.run(&p0.q, &count1)

		if (p0.blocked || p0.pc < 0 || p0.pc >= len(p0.program)) &&
			(p1.blocked || p1.pc < 0 || p1.pc >= len(p1.program)) &&
			len(p0.q) == 0 && len(p1.q) == 0 {
			break
		}
	}
	return fmt.Sprintf("%d", count1)
}

type SolverComputer struct {
	program [][]string
	pc      int
	regs    map[string]int
	q       []int
	blocked bool
}

func (d Solver) newComputer(input string, id int) *SolverComputer {
	prog := [][]string{}
	for _, line := range strings.Split(input, "\n") {
		line = strings.TrimSpace(line)
		if line != "" {
			prog = append(prog, strings.Fields(line))
		}
	}
	return &SolverComputer{program: prog, regs: map[string]int{"p": id}}
}

func (c *SolverComputer) getValue(s string) int {
	if val, err := strconv.Atoi(s); err == nil {
		return val
	}
	return c.regs[s]
}

func (c *SolverComputer) set(reg string, val int) { c.regs[reg] = val }

func (c *SolverComputer) run(out *[]int, count *int) {
	c.blocked = false
	for c.pc >= 0 && c.pc < len(c.program) {
		inst := c.program[c.pc]
		switch inst[0] {
		case "snd":
			*out = append(*out, c.getValue(inst[1]))
			if count != nil {
				*count++
			}
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
			c.set(inst[1], c.q[0])
			c.q = c.q[1:]
		case "jgz":
			if c.getValue(inst[1]) > 0 {
				c.pc += c.getValue(inst[2]) - 1
			}
		}
		c.pc++
	}
}
