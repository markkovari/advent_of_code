package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day05 struct{}

func (d Day05) Part1(input string) string {
	jumps := parse(input)
	current, steps := 0, 0
	for current >= 0 && current < len(jumps) {
		jump := jumps[current]
		jumps[current]++
		current += jump
		steps++
	}
	return fmt.Sprintf("%d", steps)
}

func (d Day05) Part2(input string) string {
	jumps := parse(input)
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

func parse(input string) []int {
	var nums []int
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		val, _ := strconv.Atoi(line)
		nums = append(nums, val)
	}
	return nums
}

func main() {
	common.Run(2017, 5, Day05{})
}
