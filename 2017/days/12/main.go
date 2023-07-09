package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day12First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("12")
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

func Day12Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("12")
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
	graph := makeGraphFromInput(input)

	var count int
	for k := range graph {
		if dfsCanReachTarget(graph, k, 0, map[int]bool{}) {
			count++
		}
	}

	return count
}

func part2(input string) int {
	graph := makeGraphFromInput(input)

	var groupCount int
	hasBeenGrouped := map[int]bool{}

	for target := range graph {
		if !hasBeenGrouped[target] {
			for k := range graph {
				if k != target && !hasBeenGrouped[k] {
					if dfsCanReachTarget(graph, k, target, map[int]bool{}) {
						hasBeenGrouped[k] = true
					}
				}
			}
			groupCount++
		}
	}

	return groupCount
}

func makeGraphFromInput(input string) map[int][]int {
	lines := strings.Split(input, "\n")
	graph := make(map[int][]int, len(lines))
	for _, l := range lines {
		parts := strings.Split(l, " <-> ")
		ID := cast.ToInt(parts[0])
		for _, child := range strings.Split(parts[1], ", ") {
			graph[ID] = append(graph[ID], cast.ToInt(child))
		}
	}
	return graph
}

func dfsCanReachTarget(graph map[int][]int, entry int, target int, visited map[int]bool) bool {
	if visited[entry] {
		return false
	}
	visited[entry] = true

	for _, child := range graph[entry] {
		if child == target || dfsCanReachTarget(graph, child, target, visited) {
			return true
		}
	}
	return false
}
