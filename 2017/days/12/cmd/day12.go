package main

import (
	"fmt"
	"strings"
	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day12 struct{}

func (d Day12) Part1(input string) string {
	graph := parse(input)
	visited := make(map[int]bool)
	dfs(0, graph, visited)
	return fmt.Sprintf("%d", len(visited))
}

func (d Day12) Part2(input string) string {
	graph := parse(input)
	visited := make(map[int]bool)
	groups := 0
	for i := 0; i < len(graph); i++ {
		if !visited[i] {
			groups++
			dfs(i, graph, visited)
		}
	}
	return fmt.Sprintf("%d", groups)
}

func parse(input string) [][]int {
	lines := strings.Split(strings.TrimSpace(input), "\n")
	graph := make([][]int, len(lines))
	for i, line := range lines {
		parts := strings.Split(line, " <-> ")
		targets := strings.Split(parts[1], ", ")
		for _, t := range targets {
			var target int
			fmt.Sscanf(t, "%d", &target)
			graph[i] = append(graph[i], target)
		}
	}
	return graph
}

func dfs(current int, graph [][]int, visited map[int]bool) {
	visited[current] = true
	for _, neighbor := range graph[current] {
		if !visited[neighbor] {
			dfs(neighbor, graph, visited)
		}
	}
}

func main() { common.Run(2017, 12, Day12{}) }
