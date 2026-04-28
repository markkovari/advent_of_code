package day19

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	res, _ := d.solve(input)
	return res
}

func (d Solver) Part2(input string) string {
	_, steps := d.solve(input)
	return fmt.Sprintf("%d", steps)
}

func (d Solver) solve(input string) (string, int) {
	diagram := strings.Split(input, "\n")
	path := ""
	steps := 0
	r, c := 0, strings.Index(diagram[0], "|")
	dr, dc := 1, 0

	for {
		if r < 0 || r >= len(diagram) || c < 0 || c >= len(diagram[r]) || diagram[r][c] == ' ' {
			break
		}
		steps++
		char := diagram[r][c]

		if char == '+' {
			if dr != 0 { // Vertical -> Horizontal
				dr = 0
				if c > 0 && diagram[r][c-1] != ' ' {
					dc = -1
				} else {
					dc = 1
				}
			} else { // Horizontal -> Vertical
				dc = 0
				if r > 0 && diagram[r-1][c] != ' ' {
					dr = -1
				} else {
					dr = 1
				}
			}
		} else if char != '|' && char != '-' {
			path += string(char)
		}
		r += dr
		c += dc
	}
	return path, steps
}
