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
	cursor := 0
	state := "A"

	for i := 0; i < steps; i++ {
		val := tape[cursor]
		r := states[state][val]
		tape[cursor] = r.valToWrite
		if r.direction == "left" {
			cursor--
		} else {
			cursor++
		}
		state = r.nextState
	}
	count := 0
	for _, v := range tape {
		if v == 1 {
			count++
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day25) Part2(input string) string {
	return "2017 Day 25 finished!"
}

type rule struct {
	valToWrite int
	nextState  string
	direction  string
}

func parse(input string) (int, map[string][2]rule) {
	blocks := strings.Split(strings.TrimSpace(input), "\n\n")
	var steps int
	lines0 := strings.Split(blocks[0], "\n")
	fmt.Sscanf(lines0[1], "Perform a diagnostic checksum after %d steps.", &steps)

	states := make(map[string][2]rule)
	for _, block := range blocks[1:] {
		lines := strings.Split(block, "\n")
		var name string
		fmt.Sscanf(strings.TrimSpace(lines[0]), "In state %1s:", &name)

		var r0, r1 rule
		// Value 0
		fmt.Sscanf(strings.Trim(lines[2], " -.:"), "Write the value %d", &r0.valToWrite)
		if strings.Contains(lines[3], "left") {
			r0.direction = "left"
		} else {
			r0.direction = "right"
		}
		fmt.Sscanf(strings.Trim(lines[4], " -.:"), "Continue with state %1s", &r0.nextState)

		// Value 1
		fmt.Sscanf(strings.Trim(lines[6], " -.:"), "Write the value %d", &r1.valToWrite)
		if strings.Contains(lines[7], "left") {
			r1.direction = "left"
		} else {
			r1.direction = "right"
		}
		fmt.Sscanf(strings.Trim(lines[8], " -.:"), "Continue with state %1s", &r1.nextState)

		states[name] = [2]rule{r0, r1}
	}
	return steps, states
}

func main() {
	common.Run(2017, 25, Day25{})
}
