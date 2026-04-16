package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day24 struct{}

func (d Day24) Part1(input string) string {
    return fmt.Sprintf("%d", solve(input, 3))
}

func (d Day24) Part2(input string) string {
    return fmt.Sprintf("%d", solve(input, 4))
}

func solve(input string, groups int) int64 {
	weights := []int64{}
	for _, l := range strings.Split(strings.TrimSpace(input), "\n") {
		var w int64; fmt.Sscanf(l, "%d", &w); weights = append(weights, w)
	}
	totalWeight := int64(0)
	for _, w := range weights { totalWeight += w }
	target := totalWeight / int64(groups)

	var minQE int64 = math.MaxInt64
	var findGroups func(int, int64, int64, int64, int)
	findGroups = func(idx int, currentWeight, currentQE int64, count int64, depth int) {
		if currentWeight == target {
			if count < minQE { minQE = currentQE }
			return
		}
		if currentWeight > target || idx == len(weights) { return }
		findGroups(idx+1, currentWeight+weights[idx], currentQE*weights[idx], count+1, depth+1)
		findGroups(idx+1, currentWeight, currentQE, count, depth)
	}
    // Simple exhaustive backtracking
	return minQE
}

func main() { common.Run(2017, 24, Day24{}) }
EOF
