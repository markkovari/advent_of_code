package main

import (
	"math"
	"os"
	"strconv"
	"strings"
)

func read02File(path string) ([][]int, error) {
	dat, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(string(dat), "\n")
	matrix := make([][]int, 0)

	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		row := make([]int, 0)
		for _, part := range parts {
			value, err := strconv.Atoi(part)
			if err != nil {
				continue
			}
			row = append(row, value)
		}
		matrix = append(matrix, row)
	}
	return matrix, nil
}

func countSafeLevels(levels [][]int) int {
	count := 0
	for _, row := range levels {
		if isLevelSafe(row) {
			count++
		}
	}
	return count
}

func isLevelSafe(row []int) bool {
	if len(row) < 3 {
		return false
	}
	increasing := row[0] < row[1]
	decreasing := !increasing
	for j := 0; j < len(row)-1; j++ {
		current, next := row[j], row[j+1]
		difference := int(math.Abs(float64(current) - float64(next)))
		if increasing && current > next {
			return false
		} else if decreasing && next > current {
			return false
		} else if difference > 3 || difference < 1 {
			return false
		}
	}
	return true
}
