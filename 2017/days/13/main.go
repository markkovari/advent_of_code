package days

import (
	"fmt"
	"math"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day13First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("13")
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

func Day13Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("13")
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
	freqs := parseInput(input)
	var severity int
	for _, freq := range freqs {
		depthIndex := freq[0]
		frequencyToZeroIndex := (freq[1] - 1) * 2

		if depthIndex%frequencyToZeroIndex == 0 {
			severity += depthIndex * freq[1]
		}
	}
	return severity
}

func part2(input string) int {
	freqs := parseInput(input)

	for delay := 0; delay < math.MaxInt32; delay++ {
		var gotCaught bool
		for _, freq := range freqs {
			depthIndex := freq[0]
			frequencyToZeroIndex := (freq[1] - 1) * 2
			if (depthIndex+delay)%frequencyToZeroIndex == 0 {
				gotCaught = true
			}
		}
		if !gotCaught {
			return delay
		}
	}

	panic("loop ended, increase limit?")
}

func parseInput(content string) [][2]int {
	result := make([][2]int, 0)
	for _, line := range strings.Split(content, "\n") {
		var depth, rng int
		fmt.Sscanf(line, "%d: %d", &depth, &rng)
		result = append(result, [2]int{depth, rng})
	}
	return result
}
