package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day01 struct{}

func (d Day01) Part1(input string) string {
	input = strings.TrimSpace(input)
	sum := 0
	for i := 0; i < len(input); i++ {
		next := (i + 1) % len(input)
		if input[i] == input[next] {
			sum += int(input[i] - '0')
		}
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day01) Part2(input string) string {
	input = strings.TrimSpace(input)
	sum := 0
	length := len(input)
	step := length / 2
	for i := 0; i < length; i++ {
		next := (i + step) % length
		if input[i] == input[next] {
			sum += int(input[i] - '0')
		}
	}
	return fmt.Sprintf("%d", sum)
}

func main() {
	common.Run(2017, 1, Day01{})
}
