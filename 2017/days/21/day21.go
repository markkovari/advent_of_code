package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day21 struct{}

func (d Day21) Part1(input string) string {
	return fmt.Sprintf("%d", solve(input, 5))
}

func (d Day21) Part2(input string) string {
	return fmt.Sprintf("%d", solve(input, 18))
}

func solve(input string, iterations int) int {
	rules := parse(input)
	mapping := newMap(rules)
	grid := [][]bool{
		{false, true, false},
		{false, false, true},
		{true, true, true},
	}
	
	for i := 0; i < iterations; i++ {
		grid = expand(mapping, grid)
	}
	return count(grid)
}

type pattern [][]bool

func count(grid [][]bool) int {
	c := 0
	for _, row := range grid {
		for _, b := range row {
			if b { c++ }
		}
	}
	return c
}

func expand(mapping map[uint16][][]bool, grid [][]bool) [][]bool {
	size := len(grid)
	step := 2
	if size % 3 == 0 { step = 3 }
	
	newSize := (size / step) * (step + 1)
	newGrid := make([][]bool, newSize)
	for i := range newGrid { newGrid[i] = make([]bool, newSize) }

	for y := 0; y < size/step; y++ {
		for x := 0; x < size/step; x++ {
			sub := make([][]bool, step)
			for i := 0; i < step; i++ {
				sub[i] = grid[y*step+i][x*step : x*step+step]
			}
			replacement := mapping[encode(sub)]
			for i := 0; i < step+1; i++ {
				for j := 0; j < step+1; j++ {
					newGrid[y*(step+1)+i][x*(step+1)+j] = replacement[i][j]
				}
			}
		}
	}
	return newGrid
}

func newMap(rules []rule) map[uint16][][]bool {
	m := make(map[uint16][][]bool)
	for _, r := range rules {
		for i := 0; i < 4; i++ {
			p := rotate(r.input, i)
			m[encode(p)] = r.output
			m[encode(flip(p))] = r.output
		}
	}
	return m
}

func encode(in [][]bool) (out uint16) {
	if len(in) > 2 { out |= (1 << 15) }
	var i uint
	for _, row := range in {
		for _, b := range row {
			if b { out |= (1 << i) }
			i++
		}
	}
	return
}

func rotate(in [][]bool, n int) [][]bool {
	if n == 0 { return in }
	size := len(in)
	out := make([][]bool, size)
	for i := range out { out[i] = make([]bool, size) }
	for r := 0; r < size; r++ {
		for c := 0; c < size; c++ {
			out[c][size-1-r] = in[r][c]
		}
	}
	return rotate(out, n-1)
}

func flip(in [][]bool) [][]bool {
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

type rule struct {
	input, output [][]bool
}

func parse(input string) []rule {
	var rules []rule
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		parts := strings.Split(line, " => ")
		rules = append(rules, rule{input: parsePattern(parts[0]), output: parsePattern(parts[1])})
	}
	return rules
}

func parsePattern(s string) [][]bool {
	parts := strings.Split(s, "/")
	var p [][]bool
	for _, row := range parts {
		r := make([]bool, len(row))
		for i, c := range row { r[i] = (c == '#') }
		p = append(p, r)
	}
	return p
}

func main() { common.Run(2017, 21, Day21{}) }
