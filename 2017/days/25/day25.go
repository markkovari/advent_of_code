package main

import (
	"fmt"
	"strings"
	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day25 struct{}

func (d Day25) Part1(input string) string {
	steps, states := parse(input)
	tape := make(map[int]int)
	cursor, state := 0, "A"

	for i := 0; i < steps; i++ {
		val := tape[cursor]
		rule := states[state][val]
		tape[cursor] = rule.valToWrite
		if rule.direction == "left" { cursor-- } else { cursor++ }
		state = rule.nextState
	}
	count := 0
	for _, v := range tape { if v == 1 { count++ } }
	return fmt.Sprintf("%d", count)
}

func (d Day25) Part2(input string) string { return "Done!" }

type rule struct { valToWrite, nextState int; direction string }

func parse(input string) (int, map[string][2]rule) {
	blocks := strings.Split(input, "\n\n")
	var steps int
	fmt.Sscanf(strings.Split(blocks[0], "\n")[1], "Perform a diagnostic checksum after %d steps.", &steps)
	states := make(map[string][2]rule)
	for _, block := range blocks[1:] {
		lines := strings.Split(block, "\n")
		var name string
		fmt.Sscanf(lines[0], "In state %1s:", &name)
		var r0, r1 rule
		fmt.Sscanf(strings.Trim(lines[2], " -."), "Write the value %d", &r0.valToWrite)
		fmt.Sscanf(strings.Trim(lines[3], " -."), "Move one slot to the %s", &r0.direction)
		fmt.Sscanf(strings.Trim(lines[4], " -."), "Continue with state %1s", &r0.nextState)
		fmt.Sscanf(strings.Trim(lines[6], " -."), "Write the value %d", &r1.valToWrite)
		fmt.Sscanf(strings.Trim(lines[7], " -."), "Move one slot to the %s", &r1.direction)
		fmt.Sscanf(strings.Trim(lines[8], " -."), "Continue with state %1s", &r1.nextState)
		states[name] = [2]rule{r0, r1}
	}
	return steps, states
}

func main() { common.Run(2017, 25, Day25{}) }
