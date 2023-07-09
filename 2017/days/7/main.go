package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

type Node struct {
	Name   string
	Weight int
	Edges  []string
}

func Day7First(isProd bool) (string, error) {
	example, prod, err := utils.ReadTextsOfDay("7")
	if err != nil {
		return "", err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part1(input), nil
}

func Day7Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("7")
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

func part1(input string) string {
	graph := parseInput(input)

	allNames := map[string]bool{}
	for k := range graph {
		allNames[k] = true
	}

	// iterate through all graph edges and remove any dependants from allNames
	// map b/c that cannot be at the bottom of the stack
	for _, node := range graph {
		for _, name := range node.Edges {
			delete(allNames, name)
		}
	}

	if len(allNames) != 1 {
		panic("Expected one name left, got" + cast.ToString(len(allNames)))
	}

	var nameAtBottom string
	for name := range allNames {
		nameAtBottom = name
	}
	return nameAtBottom
}

func part2(input string) int {
	graph := parseInput(input)
	currentNode := part1(input)
	weightCalculator := calcWeight(graph)

	var siblings []string
	for indexToExit := 0; indexToExit < len(graph); indexToExit++ {
		weightToDependents := map[int][]string{}
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
					return graph[currentNode].Weight - (currentWeight - desiredWeight)
				}
			}
		} else {
			panic("unhandled case, weightToDependents == 0")
		}
	}

	panic("something's wrong in the loop...")
}

func calcWeight(graph map[string]Node) func(string) int {
	memo := make(map[string]int, len(graph))

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

func parseInput(input string) map[string]Node {
	lines := strings.Split(input, "\n")
	graph := make(map[string]Node, len(lines))
	for _, l := range lines {
		parts := strings.Split(l, " -> ")

		leftParts := strings.Split(parts[0], " ")
		name := leftParts[0]
		weight := cast.ToInt(leftParts[1][1 : len(leftParts[1])-1])

		var edges []string
		if len(parts) == 2 {
			edges = strings.Split(parts[1], ", ")
			if len(edges) == 1 {
				panic("Assumed no nodes have exactly one dependant, but got 1 for node: " + name)
			}
		}
		graph[name] = Node{
			Name:   name,
			Weight: weight,
			Edges:  edges,
		}
	}

	return graph
}
