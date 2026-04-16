package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day24 struct{}

func (d Day24) Part1(input string) string {
	edges := getEdges(strings.TrimSpace(input))
	bridge := [][2]int{{0, 0}}
	usedEdges := make(map[int]bool)
	bestStrength, _ := backtrackBridge(bridge, edges, usedEdges)
	return fmt.Sprintf("%d", bestStrength)
}

func (d Day24) Part2(input string) string {
	edges := getEdges(strings.TrimSpace(input))
	bridge := [][2]int{{0, 0}}
	usedEdges := make(map[int]bool)
	_, longestBridge := backtrackBridge(bridge, edges, usedEdges)
	return fmt.Sprintf("%d", calcStrengthOfBridge(longestBridge))
}

func backtrackBridge(bridge [][2]int, edges [][2]int, usedEdges map[int]bool) (bestStrength int, longestBridge [][2]int) {
	lastVal := bridge[len(bridge)-1][1]
	for i, edge := range edges {
		if !usedEdges[i] {
			if edge[0] == lastVal || edge[1] == lastVal {
				clonedEdge := edge
				if clonedEdge[0] != lastVal {
					clonedEdge[0], clonedEdge[1] = clonedEdge[1], clonedEdge[0]
				}
				bridge = append(bridge, clonedEdge)
				usedEdges[i] = true

				subStrength, subLongestBridge := backtrackBridge(bridge, edges, usedEdges)

				currentStrength := calcStrengthOfBridge(bridge)
				if currentStrength + subStrength > bestStrength {
					bestStrength = currentStrength + subStrength
				}

				if len(subLongestBridge) > len(longestBridge) ||
					(len(subLongestBridge) == len(longestBridge) &&
						calcStrengthOfBridge(subLongestBridge) > calcStrengthOfBridge(longestBridge)) {
					longestBridge = append([][2]int{}, subLongestBridge...)
				}
				
				// Also check current bridge as potentially the longest
				if len(bridge) > len(longestBridge) || 
					(len(bridge) == len(longestBridge) && 
						calcStrengthOfBridge(bridge) > calcStrengthOfBridge(longestBridge)) {
					longestBridge = append([][2]int{}, bridge...)
				}

				usedEdges[i] = false
				bridge = bridge[:len(bridge)-1]
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
		if line == "" { continue }
		var pair [2]int
		fmt.Sscanf(line, "%d/%d", &pair[0], &pair[1])
		edges = append(edges, pair)
	}
	return edges
}

func main() {
	common.Run(2017, 24, Day24{})
}
