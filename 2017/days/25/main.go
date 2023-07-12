package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day25First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("25")
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

func part1(input string) int {
	steps, stateRules := parseInput(input)

	bigArray := make([]int, steps)
	index := steps / 2
	currentStateName := "A"

	for i := 0; i < steps; i++ {
		currentVal := bigArray[index]
		rulesToFollow := stateRules[currentStateName][currentVal]
		bigArray[index] = rulesToFollow.valToWrite
		if rulesToFollow.direction == "left" {
			index--
		} else {
			index++
		}
		currentStateName = rulesToFollow.nextState
	}

	return sumIntSlice(bigArray)
}

func sumIntSlice(s []int) int {
	sum := 0
	for _, v := range s {
		sum += v
	}
	return sum
}

type ruleset struct {
	name       string
	valToWrite int
	direction  string
	nextState  string
}

func parseInput(input string) (steps int, states map[string][2]ruleset) {
	blocks := strings.Split(input, "\n\n")

	fmt.Sscanf(strings.Split(blocks[0], "\n")[1], "Perform a diagnostic checksum after %d steps.", &steps)

	states = map[string][2]ruleset{}
	for _, block := range blocks[1:] {
		lines := strings.Split(block, "\n")
		var stateName string
		fmt.Sscanf(lines[0], "In state %1s:", &stateName)

		rulesIfZero := ruleset{name: stateName}
		fmt.Sscanf(strings.Trim(lines[2], " -."), "Write the value %d", &rulesIfZero.valToWrite)
		fmt.Sscanf(strings.Trim(lines[3], " -."), "Move one slot to the %s", &rulesIfZero.direction)
		fmt.Sscanf(strings.Trim(lines[4], " -."), "Continue with state %1s", &rulesIfZero.nextState)

		rulesIfOne := ruleset{name: stateName}
		fmt.Sscanf(strings.Trim(lines[6], " -."), "Write the value %d", &rulesIfOne.valToWrite)
		fmt.Sscanf(strings.Trim(lines[7], " -."), "Move one slot to the %s", &rulesIfOne.direction)
		fmt.Sscanf(strings.Trim(lines[8], " -."), "Continue with state %1s", &rulesIfOne.nextState)

		states[stateName] = [2]ruleset{rulesIfZero, rulesIfOne}
	}

	return steps, states
}
