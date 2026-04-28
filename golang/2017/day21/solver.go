package day21

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	return fmt.Sprintf("%d", d.solve(input, 5))
}

func (d Solver) Part2(input string) string {
	return fmt.Sprintf("%d", d.solve(input, 18))
}

func (d Solver) solve(input string, iterations int) int {
	rules := d.parse(input)
	mapping := d.newMap(rules)
	grid := [][]bool{
		{false, true, false},
		{false, false, true},
		{true, true, true},
	}

	for i := 0; i < iterations; i++ {
		grid = d.expand(mapping, grid)
	}
	return d.count(grid)
}

type Solverpattern [][]bool

func (d Solver) count(grid [][]bool) int {
	c := 0
	for _, row := range grid {
		for _, b := range row {
			if b {
				c++
			}
		}
	}
	return c
}

func (d Solver) expand(mapping map[uint16][][]bool, grid [][]bool) [][]bool {
	size := len(grid)
	step := 3
	if size%2 == 0 {
		step = 2
	}

	newSize := (size / step) * (step + 1)
	newGrid := make([][]bool, newSize)
	for i := range newGrid {
		newGrid[i] = make([]bool, newSize)
	}

	for y := 0; y < size/step; y++ {
		for x := 0; x < size/step; x++ {
			sub := make([][]bool, step)
			for i := 0; i < step; i++ {
				sub[i] = grid[y*step+i][x*step : x*step+step]
			}
			replacement := mapping[d.encode(sub)]
			for i := 0; i < step+1; i++ {
				for j := 0; j < step+1; j++ {
					newGrid[y*(step+1)+i][x*(step+1)+j] = replacement[i][j]
				}
			}
		}
	}
	return newGrid
}

func (d Solver) newMap(rules []Solverrule) map[uint16][][]bool {
	m := make(map[uint16][][]bool)
	for _, r := range rules {
		for i := 0; i < 4; i++ {
			p := d.rotate(r.input, i)
			m[d.encode(p)] = r.output
			m[d.encode(d.flip(p))] = r.output
		}
	}
	return m
}

func (d Solver) encode(in [][]bool) (out uint16) {
	if len(in) > 2 {
		out |= (1 << 15)
	}
	var i uint
	for _, row := range in {
		for _, b := range row {
			if b {
				out |= (1 << i)
			}
			i++
		}
	}
	return
}

func (d Solver) rotate(in [][]bool, n int) [][]bool {
	if n == 0 {
		return in
	}
	size := len(in)
	out := make([][]bool, size)
	for i := range out {
		out[i] = make([]bool, size)
	}
	for r := 0; r < size; r++ {
		for c := 0; c < size; c++ {
			out[c][size-1-r] = in[r][c]
		}
	}
	return d.rotate(out, n-1)
}

func (d Solver) flip(in [][]bool) [][]bool {
	size := len(in)
	out := make([][]bool, size)
	for r := 0; r < size; r++ {
		out[r] = make([]bool, size)
		for c := 0; c < size; c++ {
			out[r][c] = in[r][size-1-c]
		}
	}
	return out
}

type Solverrule struct {
	input, output [][]bool
}

func (d Solver) parse(input string) []Solverrule {
	var rules []Solverrule
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		parts := strings.Split(line, " => ")
		rules = append(rules, Solverrule{input: d.parsePattern(parts[0]), output: d.parsePattern(parts[1])})
	}
	return rules
}

func (d Solver) parsePattern(s string) [][]bool {
	parts := strings.Split(s, "/")
	var p [][]bool
	for _, row := range parts {
		r := make([]bool, len(row))
		for i, c := range row {
			r[i] = (c == '#')
		}
		p = append(p, r)
	}
	return p
}
