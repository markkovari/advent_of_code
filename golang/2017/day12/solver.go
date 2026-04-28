package day12

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	graph := d.parse(input)
	visited := make(map[int]bool)
	d.dfs(0, graph, visited)
	return fmt.Sprintf("%d", len(visited))
}

func (d Solver) Part2(input string) string {
	graph := d.parse(input)
	visited := make(map[int]bool)
	groups := 0
	for i := 0; i < len(graph); i++ {
		if !visited[i] {
			groups++
			d.dfs(i, graph, visited)
		}
	}
	return fmt.Sprintf("%d", groups)
}

func (d Solver) parse(input string) [][]int {
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

func (d Solver) dfs(current int, graph [][]int, visited map[int]bool) {
	visited[current] = true
	for _, neighbor := range graph[current] {
		if !visited[neighbor] {
			d.dfs(neighbor, graph, visited)
		}
	}
}
