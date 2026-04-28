package day09

import (
	"fmt"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	score, _ := d.solve(input)
	return fmt.Sprintf("%d", score)
}

func (d Solver) Part2(input string) string {
	_, garbage := d.solve(input)
	return fmt.Sprintf("%d", garbage)
}

func (d Solver) solve(input string) (int, int) {
	score, depth, garbage, totalGarbage := 0, 0, false, 0
	skip := false
	for _, c := range input {
		if skip {
			skip = false
			continue
		}
		if c == '!' {
			skip = true
			continue
		}
		if garbage {
			if c == '>' {
				garbage = false
			} else {
				totalGarbage++
			}
		} else {
			if c == '<' {
				garbage = true
			} else if c == '{' {
				depth++
				score += depth
			} else if c == '}' {
				depth--
			}
		}
	}
	return score, totalGarbage
}
