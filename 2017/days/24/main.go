package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day24First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("24")
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

func Day24Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("24")
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
	return magneticMoat(input, 1)
}

func part2(input string) int {
	return magneticMoat(input, 2)
}

func magneticMoat(input string, part int) int {
	edges := getEdges(input)

	bridge := [][2]int{
		{0, 0},
	}

	usedEdges := map[[2]int]bool{}
	for _, edge := range edges {
		usedEdges[edge] = false
	}

	bestStrength, longestBridge := backtrackBridge(bridge, usedEdges)
	if part == 1 {
		return bestStrength
	}
	return calcStrengthOfBridge(longestBridge)
}

func backtrackBridge(bridge [][2]int, usedEdges map[[2]int]bool) (bestStrength int, longestBridge [][2]int) {
	lastVal := bridge[len(bridge)-1][1]
	for edge, isUsed := range usedEdges {
		if !isUsed {
			clonedEdge := edge
			if clonedEdge[0] != lastVal {
				clonedEdge[0], clonedEdge[1] = clonedEdge[1], clonedEdge[0]
			}
			if clonedEdge[0] == lastVal {
				bridge = append(bridge, clonedEdge)
				usedEdges[edge] = true
				strength := clonedEdge[0] + clonedEdge[1]

				subStrength, subLongestBridge := backtrackBridge(bridge, usedEdges)

				strength += subStrength

				if len(bridge) > len(longestBridge) ||
					(len(bridge) == len(longestBridge) &&
						calcStrengthOfBridge(bridge) > calcStrengthOfBridge(longestBridge)) {
					longestBridge = append([][2]int{}, bridge...)
				}
				if len(subLongestBridge) > len(longestBridge) ||
					(len(subLongestBridge) == len(longestBridge) &&
						calcStrengthOfBridge(subLongestBridge) > calcStrengthOfBridge(longestBridge)) {
					longestBridge = append([][2]int{}, subLongestBridge...)
				}

				usedEdges[edge] = false
				bridge = bridge[:len(bridge)-1]
				if strength > bestStrength {
					bestStrength = strength
				}
			}
		}
	}

	return bestStrength, longestBridge
}

func calcStrengthOfBridge(bridge [][2]int) int {
	var sum int
	for _, edge := range bridge {
		sum += edge[0] + edge[1]
	}
	return sum
}

func getEdges(input string) (edges [][2]int) {
	for _, line := range strings.Split(input, "\n") {
		var pair [2]int
		fmt.Sscanf(line, "%d/%d", &pair[0], &pair[1])
		edges = append(edges, pair)
	}
	return edges
}
