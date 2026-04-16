package main

import (
	"fmt"
	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day09 struct{}

func (d Day09) Part1(input string) string {
	score, _ := solve(input)
	return fmt.Sprintf("%d", score)
}

func (d Day09) Part2(input string) string {
	_, garbage := solve(input)
	return fmt.Sprintf("%d", garbage)
}

func solve(input string) (int, int) {
	score, depth, garbage, totalGarbage := 0, 0, false, 0
	skip := false
	for _, c := range input {
		if skip { skip = false; continue }
		if c == '!' { skip = true; continue }
		if garbage {
			if c == '>' { garbage = false } else { totalGarbage++ }
		} else {
			if c == '<' { garbage = true } else if c == '{' { depth++; score += depth } else if c == '}' { depth-- }
		}
	}
	return score, totalGarbage
}

func main() { common.Run(2017, 9, Day09{}) }
