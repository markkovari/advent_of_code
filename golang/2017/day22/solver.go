package day22

import (
	"fmt"
	"strings"
)

type Solver struct{}

type SolverinfectedState int

const (
	day22Clean SolverinfectedState = iota
	day22Weakened
	day22Infected
	day22Flagged
)

func (d Solver) Part1(input string) string {
	state := d.newState(input)
	current := [2]int{len(strings.Split(input, "\n")) / 2, len(strings.Split(input, "\n")) / 2}
	dir, countBursts := 0, 0
	dirs := [][2]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	for i := 0; i < 10000; i++ {
		if state[current] == day22Infected {
			dir = (dir + 1) % 4
			state[current] = day22Clean
		} else {
			dir = (dir + 3) % 4
			state[current] = day22Infected
			countBursts++
		}
		current = [2]int{current[0] + dirs[dir][0], current[1] + dirs[dir][1]}
	}
	return fmt.Sprintf("%d", countBursts)
}

func (d Solver) Part2(input string) string {
	state := d.newState(input)
	current := [2]int{len(strings.Split(input, "\n")) / 2, len(strings.Split(input, "\n")) / 2}
	dir, countBursts := 0, 0
	dirs := [][2]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

	for i := 0; i < 10000000; i++ {
		switch state[current] {
		case day22Clean:
			dir = (dir + 3) % 4
			state[current] = day22Weakened
		case day22Weakened:
			state[current] = day22Infected
			countBursts++
		case day22Infected:
			dir = (dir + 1) % 4
			state[current] = day22Flagged
		case day22Flagged:
			dir = (dir + 2) % 4
			state[current] = day22Clean
		}
		current = [2]int{current[0] + dirs[dir][0], current[1] + dirs[dir][1]}
	}
	return fmt.Sprintf("%d", countBursts)
}

func (d Solver) newState(input string) map[[2]int]SolverinfectedState {
	ans := map[[2]int]SolverinfectedState{}
	for r, line := range strings.Split(input, "\n") {
		for c, v := range line {
			if v == '#' {
				ans[[2]int{r, c}] = day22Infected
			} else if v == '.' {
				ans[[2]int{r, c}] = day22Clean
			}
		}
	}
	return ans
}
