package day07

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

type Solver struct{}

type SolverNode struct {
	Name   string
	Weight int
	Edges  []string
}

func (d Solver) Part1(input string) string {
	graph := d.parse(input)
	allNames := make(map[string]bool)
	for k := range graph {
		allNames[k] = true
	}
	for _, node := range graph {
		for _, name := range node.Edges {
			delete(allNames, name)
		}
	}
	var nameAtBottom string
	for name := range allNames {
		nameAtBottom = name
	}
	return nameAtBottom
}

func (d Solver) Part2(input string) string {
	graph := d.parse(input)
	currentNode := d.Part1(input)
	weightCalculator := d.calcWeight(graph)

	var siblings []string
	for {
		weightToDependents := make(map[int][]string)
		for _, dependentName := range graph[currentNode].Edges {
			weight := weightCalculator(dependentName)
			weightToDependents[weight] = append(weightToDependents[weight], dependentName)
		}

		if len(weightToDependents) > 1 {
			siblings = graph[currentNode].Edges
			for _, names := range weightToDependents {
				if len(names) == 1 {
					currentNode = names[0]
				}
			}
		} else if len(weightToDependents) == 1 {
			currentWeight := weightCalculator(currentNode)
			for _, sib := range siblings {
				if sib != currentNode {
					desiredWeight := weightCalculator(sib)
					return fmt.Sprintf("%d", graph[currentNode].Weight-(currentWeight-desiredWeight))
				}
			}
		} else {
			break
		}
	}
	return "0"
}

func (d Solver) parse(input string) map[string]SolverNode {
	lines := strings.Split(input, "\n")
	graph := make(map[string]SolverNode)
	re := regexp.MustCompile(`([a-z]+) \((\d+)\)(?: -> (.*))?`)
	for _, l := range lines {
		if l == "" {
			continue
		}
		caps := re.FindStringSubmatch(l)
		name := caps[1]
		weight, _ := strconv.Atoi(caps[2])
		var edges []string
		if caps[3] != "" {
			edges = strings.Split(caps[3], ", ")
		}
		graph[name] = SolverNode{Name: name, Weight: weight, Edges: edges}
	}
	return graph
}

func (d Solver) calcWeight(graph map[string]SolverNode) func(string) int {
	memo := make(map[string]int)
	var closureFunc func(string) int
	closureFunc = func(rootName string) int {
		if wt, ok := memo[rootName]; ok {
			return wt
		}
		sum := graph[rootName].Weight
		for _, dependent := range graph[rootName].Edges {
			sum += closureFunc(dependent)
		}
		memo[rootName] = sum
		return sum
	}
	return closureFunc
}
