package day15

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	return fmt.Sprintf("%d", d.duelingGenerators(input, 1))
}

func (d Solver) Part2(input string) string {
	return fmt.Sprintf("%d", d.duelingGenerators(input, 2))
}

func (d Solver) duelingGenerators(input string, part int) int {
	values := d.parse(input)
	factors := []int{16807, 48271}
	divisor := 2147483647
	criteria := []int{1, 1}
	rounds := 40000000

	if part == 2 {
		rounds = 5000000
		criteria = []int{4, 8}
	}

	judgeCount := 0
	for i := 0; i < rounds; i++ {
		values[0] = d.getNextValue(values[0], factors[0], divisor, criteria[0])
		values[1] = d.getNextValue(values[1], factors[1], divisor, criteria[1])

		if (values[0] & 0xFFFF) == (values[1] & 0xFFFF) {
			judgeCount++
		}
	}
	return judgeCount
}

func (d Solver) getNextValue(value, factor, divisor, criteria int) int {
	for {
		value = (value * factor) % divisor
		if value%criteria == 0 {
			return value
		}
	}
}

func (d Solver) parse(input string) []int {
	var ans []int
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		parts := strings.Split(line, " with ")
		var val int
		fmt.Sscanf(parts[1], "%d", &val)
		ans = append(ans, val)
	}
	return ans
}
