package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day15First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("15")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part1(input), nil
}

func Day15Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("15")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}
	return part2(input), nil
}

func part1(input string) int {
	return duelingGenerators(input, 1)
}

func part2(input string) int {
	return duelingGenerators(input, 2)
}

func duelingGenerators(input string, part int) int {
	values := parseInput(input)

	factors := []int{16807, 48271}
	divisor := 2147483647

	criteria := []int{1, 1}
	rounds := 40000000
	if part == 2 {
		rounds = 5000000
		criteria = []int{4, 8}
	}

	var judgeCount int
	for i := 0; i < rounds; i++ {
		for i, v := range values {
			values[i] = getNextValue(v, factors[i], divisor, criteria[i])
		}

		compareVal := values[0] ^ values[1]
		twoPow16 := 1 << 16
		if (compareVal % twoPow16) == 0 {
			judgeCount++
		}
	}

	return judgeCount
}
func getNextValue(value, factor, divisor, criteria int) int {
	value *= factor
	value %= divisor

	for value%criteria != 0 {
		value *= factor
		value %= divisor
	}

	return value
}

func parseInput(input string) (ans []int) {
	lines := strings.Split(input, "\n")
	for _, l := range lines {
		split := strings.Split(l, " starts with ")
		ans = append(ans, cast.ToInt(split[1]))
	}
	return ans
}
