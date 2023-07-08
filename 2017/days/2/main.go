package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"

	"github.com/spf13/cast"
)

func Day2First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("2")
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
func Day2Second(isProd bool) (int, error) {
	_, prod, err := utils.ReadTextsOfDay("2")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		// ffs not sure why AoC cannot give us the same input for both parts
		input = `5 9 2 8
9 4 7 3
3 8 6 5`
	}

	return part2(input), nil
}

func part1(input string) int {
	spreadSheet := parseInput(input)
	var sum int
	for _, row := range spreadSheet {
		sum += getDiff(row)
	}
	return sum
}

func part2(input string) int {
	spreadSheet := parseInput(input)
	var sum int
	for _, row := range spreadSheet {
		sum += getDivisible(row)
	}
	return sum

}

func getDivisible(numbers []int) int {
	for i, num := range numbers {
		for j, num2 := range numbers {
			if i == j {
				continue
			}
			if num%num2 == 0 {
				return num / num2
			}
		}
	}
	return 0
}

func getDiff(numbers []int) int {
	smallest, biggest := findSmallestAndBiggest(numbers)
	return biggest - smallest
}

// Assume that the input is a list of numbers with at least one number
func findSmallestAndBiggest(numbers []int) (int, int) {
	smallest := numbers[0]
	biggest := numbers[0]
	for _, num := range numbers {
		if num < smallest {
			smallest = num
		}
		if num > biggest {
			biggest = num
		}
	}
	return smallest, biggest
}

func parseInput(input string) (ans [][]int) {
	for _, line := range strings.Split(input, "\n") {
		currentLine := make([]int, 0)
		for _, num := range strings.Fields(line) {
			currentLine = append(currentLine, cast.ToInt(num))
		}
		ans = append(ans, currentLine)
	}
	return ans

}
