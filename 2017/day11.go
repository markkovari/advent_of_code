package main

import (
	"fmt"
	"strings"
)

type Day11 struct{}

func (d Day11) Part1(input string) string {
	d1, _ := d.solve(input)
	return fmt.Sprintf("%d", d1)
}

func (d Day11) Part2(input string) string {
	_, d2 := d.solve(input)
	return fmt.Sprintf("%d", d2)
}

func (d Day11) solve(input string) (int, int) {
	x, y, z := 0, 0, 0
	maxD := 0
	for _, dir := range strings.Split(strings.TrimSpace(input), ",") {
		switch dir {
		case "n":
			y++
			z--
		case "s":
			y--
			z++
		case "ne":
			x++
			z--
		case "sw":
			x--
			z++
		case "nw":
			x--
			y++
		case "se":
			x++
			y--
		}
		dist := (d.abs(x) + d.abs(y) + d.abs(z)) / 2
		if dist > maxD {
			maxD = dist
		}
	}
	return (d.abs(x) + d.abs(y) + d.abs(z)) / 2, maxD
}

func (d Day11) abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
