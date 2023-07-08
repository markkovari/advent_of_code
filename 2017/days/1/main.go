package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"

	"github.com/spf13/cast"
)

func Day1First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("1")
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
func Day1Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("1")
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
	digits := parseInput(input)
	var sum int
	for i := 0; i < len(digits); i++ {
		current := digits[i]
		next := digits[(i+1)%len(digits)]
		if current == next {
			sum += digits[i]
		}
	}
	return sum
}

func part2(input string) int {
	digits := parseInput(input)
	var sum int
	offset := len(digits) / 2
	for i := 0; i < len(digits); i++ {
		if digits[i] == digits[(i+offset)%len(digits)] {
			sum += digits[i]
		}
	}
	return sum
}

func parseInput(input string) (ans []int) {
	for _, num := range strings.Split(input, "") {
		ans = append(ans, cast.ToInt(num))
	}
	return ans
}
