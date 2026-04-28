package day24

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	edges := d.getEdges(strings.TrimSpace(input))
	usedEdges := make(map[int]bool)
	bestStrength, _, _ := d.backtrack(0, edges, usedEdges)
	return fmt.Sprintf("%d", bestStrength)
}

func (d Solver) Part2(input string) string {
	edges := d.getEdges(strings.TrimSpace(input))
	usedEdges := make(map[int]bool)
	_, _, longestBridgeStrength := d.backtrack(0, edges, usedEdges)
	return fmt.Sprintf("%d", longestBridgeStrength)
}

func (d Solver) backtrack(lastVal int, edges [][2]int, usedEdges map[int]bool) (maxStr, maxLen, maxLenStr int) {
	for i, edge := range edges {
		if !usedEdges[i] {
			if edge[0] == lastVal || edge[1] == lastVal {
				usedEdges[i] = true
				nextVal := edge[0]
				if nextVal == lastVal {
					nextVal = edge[1]
				}

				s, l, ls := d.backtrack(nextVal, edges, usedEdges)

				// Part 1
				if edge[0]+edge[1]+s > maxStr {
					maxStr = edge[0] + edge[1] + s
				}

				// Part 2
				curLen := 1 + l
				curLenStr := edge[0] + edge[1] + ls
				if curLen > maxLen {
					maxLen = curLen
					maxLenStr = curLenStr
				} else if curLen == maxLen {
					if curLenStr > maxLenStr {
						maxLenStr = curLenStr
					}
				}

				usedEdges[i] = false
			}
		}
	}
	return
}

func (d Solver) calcStrengthOfBridge(bridge [][2]int) int {
	var sum int
	for _, edge := range bridge {
		sum += edge[0] + edge[1]
	}
	return sum
}

func (d Solver) getEdges(input string) (edges [][2]int) {
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		var pair [2]int
		fmt.Sscanf(line, "%d/%d", &pair[0], &pair[1])
		edges = append(edges, pair)
	}
	return edges
}
