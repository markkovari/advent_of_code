package main

import (
	"fmt"
	"strings"
	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day11 struct{}

func (d Day11) Part1(input string) string {
	d1, _ := solve(input)
	return fmt.Sprintf("%d", d1)
}

func (d Day11) Part2(input string) string {
	_, d2 := solve(input)
	return fmt.Sprintf("%d", d2)
}

func solve(input string) (int, int) {
	x, y, z := 0, 0, 0
	maxD := 0
	for _, dir := range strings.Split(strings.TrimSpace(input), ",") {
		switch dir {
		case "n": y++; z--
		case "s": y--; z++
		case "ne": x++; z--
		case "sw": x--; z++
		case "nw": x--; y++
		case "se": x++; y--
		}
		dist := (abs(x) + abs(y) + abs(z)) / 2
		if dist > maxD { maxD = dist }
	}
	return (abs(x) + abs(y) + abs(z)) / 2, maxD
}

func abs(x int) int { if x < 0 { return -x }; return x }

func main() { common.Run(2017, 11, Day11{}) }
