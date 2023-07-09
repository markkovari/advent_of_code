package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day5First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("5")
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

func Day5Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("5")
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
	jumps := parseInput(input)
	current := 0
	steps := 0
	for current < len(jumps) {
		jump := jumps[current]
		jumps[current]++
		current += jump
		steps++
	}
	return steps
}

func part2(input string) int {
	jumps := parseInput(input)
	current := 0
	steps := 0
	for current < len(jumps) {
		jump := jumps[current]
		if jump >= 3 {
			jumps[current]--
		} else {
			jumps[current]++
		}
		current += jump
		steps++
	}

	return steps
}

func parseInput(input string) (values []int) {
	var numbers []int
	for _, line := range strings.Split(input, "\n") {
		numbers = append(numbers, cast.ToInt(line))
	}
	return numbers

}
