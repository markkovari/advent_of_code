package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day02 struct{}

func (d Day02) Part1(input string) string {
	levels := parse(input)
	count := 0
	for _, row := range levels {
		if isLevelSafe(row) {
			count++
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day02) Part2(input string) string {
	levels := parse(input)
	count := 0
	for _, row := range levels {
		if isLevelSafeTolerate(row) {
			count++
		}
	}
	return fmt.Sprintf("%d", count)
}

func isLevelSafe(row []int) bool {
	if len(row) < 2 {
		return false
	}
	increasing := row[0] < row[1]
	decreasing := !increasing
	for j := 0; j < len(row)-1; j++ {
		current, next := row[j], row[j+1]
		difference := int(math.Abs(float64(current) - float64(next)))
		if increasing && current >= next {
			return false
		} else if decreasing && next >= current {
			return false
		} else if difference > 3 || difference < 1 {
			return false
		}
	}
	return true
}

func isLevelSafeTolerate(row []int) bool {
	for i := 0; i < len(row); i++ {
		if isLevelSafe(skipAtFrom(i, row)) {
			return true
		}
	}
	return false
}

func skipAtFrom(index int, elements []int) []int {
	newElements := make([]int, 0)
	for i := range elements {
		if i == index { continue }
		newElements = append(newElements, elements[i])
	}
	return newElements
}

func parse(input string) [][]int {
	matrix := make([][]int, 0)
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		parts := strings.Fields(line)
		row := make([]int, 0)
		for _, part := range parts {
			value, _ := strconv.Atoi(part)
			row = append(row, value)
		}
		matrix = append(matrix, row)
	}
	return matrix
}

func main() {
	common.Run(2024, 2, Day02{})
}
