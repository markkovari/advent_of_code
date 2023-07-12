package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

var dirs = [][2]int{
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
}

type infectedState int

const (
	clean infectedState = iota
	weakened
	infected
	flagged
)

func Day22First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("22")
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

func Day22Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("22")
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
	infectedMap := newStateFromInput(input)
	mid := len(strings.Split(input, "\n")) / 2
	current := [2]int{mid, mid}

	var dirIndex, countBursts int

	for i := 0; i < 10000; i++ {
		switch infectedMap[current] {
		case infected:
			dirIndex = (dirIndex + 1) % 4
			infectedMap[current] = clean
		case clean:
			dirIndex = (dirIndex + 3) % 4
			infectedMap[current] = infected

			countBursts++
		}
		current = [2]int{current[0] + dirs[dirIndex][0], current[1] + dirs[dirIndex][1]}
	}

	return countBursts
}

func part2(input string) int {
	state := newStateFromInput(input)
	mid := len(strings.Split(input, "\n")) / 2
	current := [2]int{mid, mid}

	var dirIndex, countBursts int

	for i := 0; i < 10000000; i++ {
		switch state[current] {
		case clean:
			dirIndex = (dirIndex + 3) % 4
			state[current] = weakened
		case weakened:
			// keep going in same direction
			state[current] = infected
			countBursts++
		case infected:
			dirIndex = (dirIndex + 1) % 4
			state[current] = flagged
		case flagged:
			dirIndex = (dirIndex + 2) % 4
			state[current] = clean
		default:
			panic(fmt.Sprintf("unhandled infection type %v", state[current]))
		}
		current = [2]int{current[0] + dirs[dirIndex][0], current[1] + dirs[dirIndex][1]}
	}

	return countBursts
}

func newStateFromInput(input string) map[[2]int]infectedState {
	ans := map[[2]int]infectedState{}
	for r, line := range strings.Split(input, "\n") {
		for c, v := range strings.Split(line, "") {
			if v == "#" {
				ans[[2]int{r, c}] = infected
			} else if v == "." {
				ans[[2]int{r, c}] = clean
			}
		}
	}
	return ans
}
