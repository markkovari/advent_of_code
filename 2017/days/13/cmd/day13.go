package main

import (
	"fmt"
	"math"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day13 struct{}

func (d Day13) Part1(input string) string {
	freqs := parse(input)
	severity := 0
	for _, freq := range freqs {
		depthIndex, rng := freq[0], freq[1]
		if depthIndex%((rng-1)*2) == 0 {
			severity += depthIndex * rng
		}
	}
	return fmt.Sprintf("%d", severity)
}

func (d Day13) Part2(input string) string {
	freqs := parse(input)
	for delay := 0; delay < math.MaxInt32; delay++ {
		gotCaught := false
		for _, freq := range freqs {
			depthIndex, rng := freq[0], freq[1]
			if (depthIndex+delay)%((rng-1)*2) == 0 {
				gotCaught = true
				break
			}
		}
		if !gotCaught {
			return fmt.Sprintf("%d", delay)
		}
	}
	return "0"
}

func parse(input string) [][2]int {
	result := make([][2]int, 0)
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		var depth, rng int
		fmt.Sscanf(line, "%d: %d", &depth, &rng)
		result = append(result, [2]int{depth, rng})
	}
	return result
}

func main() {
	common.Run(2017, 13, Day13{})
}
