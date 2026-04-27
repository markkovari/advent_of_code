package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Day02 struct{}

func (d Day02) Part1(input string) string {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		numbers := d.parseRow(line)
		min, max := d.findSmallestAndBiggest(numbers)
		sum += max - min
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day02) Part2(input string) string {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		numbers := d.parseRow(line)
		sum += d.getDivisible(numbers)
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day02) parseRow(line string) []int {
	var row []int
	for _, part := range strings.Fields(line) {
		val, _ := strconv.Atoi(part)
		row = append(row, val)
	}
	return row
}

func (d Day02) findSmallestAndBiggest(numbers []int) (int, int) {
	smallest, biggest := numbers[0], numbers[0]
	for _, num := range numbers {
		if num < smallest {
			smallest = num
		}
		if num > biggest {
			biggest = num
		}
	}
	return smallest, biggest
}

func (d Day02) getDivisible(numbers []int) int {
	for i, num := range numbers {
		for j, num2 := range numbers {
			if i != j && num%num2 == 0 {
				return num / num2
			}
		}
	}
	return 0
}
