package day05

import (
	"fmt"
	"strconv"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	jumps := d.parse(input)
	current, steps := 0, 0
	for current >= 0 && current < len(jumps) {
		jump := jumps[current]
		jumps[current]++
		current += jump
		steps++
	}
	return fmt.Sprintf("%d", steps)
}

func (d Solver) Part2(input string) string {
	jumps := d.parse(input)
	current, steps := 0, 0
	for current >= 0 && current < len(jumps) {
		jump := jumps[current]
		if jump >= 3 {
			jumps[current]--
		} else {
			jumps[current]++
		}
		current += jump
		steps++
	}
	return fmt.Sprintf("%d", steps)
}

func (d Solver) parse(input string) []int {
	var nums []int
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		val, _ := strconv.Atoi(line)
		nums = append(nums, val)
	}
	return nums
}
