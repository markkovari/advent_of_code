package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day19 struct{}

func (d Day19) Part1(input string) string {
	res, _ := solve(input)
	return res
}

func (d Day19) Part2(input string) string {
	_, steps := solve(input)
	return fmt.Sprintf("%d", steps)
}

func solve(input string) (string, int) {
	diagram := strings.Split(input, "\n")
	path := ""
	steps := 1
	r, c := 0, strings.Index(diagram[0], "|")
	dr, dc := 1, 0

	for {
		r += dr
		c += dc
		steps++

		char := diagram[r][c]
		if char == ' ' { break }

		if char == '+' {
			if dr != 0 { // Vertical -> Horizontal
				dr = 0
				if c > 0 && diagram[r][c-1] != ' ' { dc = -1 } else { dc = 1 }
			} else { // Horizontal -> Vertical
				dc = 0
				if r > 0 && diagram[r-1][c] != ' ' { dr = -1 } else { dr = 1 }
			}
		} else if char != '|' && char != '-' {
			path += string(char)
		}
	}
	return path, steps
}

func main() { common.Run(2017, 19, Day19{}) }
