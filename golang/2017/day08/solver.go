package day08

import (
	"fmt"
	"strconv"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	return fmt.Sprintf("%d", d.solve(input, false))
}

func (d Solver) Part2(input string) string {
	return fmt.Sprintf("%d", d.solve(input, true))
}

func (d Solver) solve(input string, part2 bool) int {
	registers := make(map[string]int)
	maxSeen := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		p := strings.Fields(line)
		reg, op, val, condReg, condOp, condVal := p[0], p[1], p[2], p[4], p[5], p[6]
		v, _ := strconv.Atoi(val)
		cv, _ := strconv.Atoi(condVal)

		cond := false
		rVal := registers[condReg]
		switch condOp {
		case ">":
			cond = rVal > cv
		case "<":
			cond = rVal < cv
		case ">=":
			cond = rVal >= cv
		case "<=":
			cond = rVal <= cv
		case "==":
			cond = rVal == cv
		case "!=":
			cond = rVal != cv
		}

		if cond {
			if op == "inc" {
				registers[reg] += v
			} else {
				registers[reg] -= v
			}
			for _, val := range registers {
				if val > maxSeen {
					maxSeen = val
				}
			}
		}
	}
	if part2 {
		return maxSeen
	}
	maxVal := 0
	for _, val := range registers {
		if val > maxVal {
			maxVal = val
		}
	}
	return maxVal
}
