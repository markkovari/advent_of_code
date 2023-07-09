package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

var dirIndices = map[string]int{
	"n":  0,
	"ne": 1,
	"se": 2,
	"s":  3,
	"sw": 4,
	"nw": 5,
}

func Day11First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("11")
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

func Day11Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("11")
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
	return hexEd(input, 1)
}

func part2(input string) int {
	return hexEd(input, 2)
}

func hexEd(input string, part int) int {
	steps := strings.Split(input, ",")
	tallyDirections := make([]int, 6)
	var furthest int
	for _, step := range steps {
		tallyDirections[dirIndices[step]]++
		distanceFromStart := getDistanceFromOrigin(tallyDirections)
		furthest = max(furthest, distanceFromStart)
	}

	if part == 1 {
		return getDistanceFromOrigin(tallyDirections)
	}
	return furthest
}

func getDistanceFromOrigin(tally []int) int {

	for i := range tally {
		if tally[i] != 0 {
			oppositeIndex := (i + 3) % 6
			smaller := min(tally[oppositeIndex], tally[i])
			tally[oppositeIndex] -= smaller
			tally[i] -= smaller
		}
	}

	for i := range tally {
		toLeft := (i + 5) % 6
		toRight := (i + 1) % 6
		if tally[toLeft] > 0 && tally[toRight] > 0 {
			smaller := min(tally[toLeft], tally[toRight])
			tally[toLeft] -= smaller
			tally[toRight] -= smaller
			tally[i] += smaller
		}
	}

	distanceFromOrigin := sumIntSlice(tally)

	return distanceFromOrigin
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
func sumIntSlice(nums []int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	return sum
}
