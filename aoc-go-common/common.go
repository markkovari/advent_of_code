package common

import (
	"fmt"
	"os"
	"time"
)

type Solver interface {
	Part1(input string) string
	Part2(input string) string
}

func Run(year, day int, solver Solver) {
	inputPath := findInput(year, day)
	input, err := os.ReadFile(inputPath)
	if err != nil {
		fmt.Printf("Error reading input for %d Day %d: %v\n", year, day, err)
		return
	}

	inputStr := string(input)

	start1 := time.Now()
	res1 := solver.Part1(inputStr)
	dur1 := time.Since(start1)
	fmt.Printf("--- Year %d Day %d ---\n", year, day)
	fmt.Printf("Part 1: %s (%v)\n", res1, dur1)

	start2 := time.Now()
	res2 := solver.Part2(inputStr)
	dur2 := time.Since(start2)
	fmt.Printf("Part 2: %s (%v)\n", res2, dur2)
}

func findInput(year, day int) string {
	paths := []string{
		fmt.Sprintf("../../%d/inputs/%d/prod.txt", year, day),
		fmt.Sprintf("%d/inputs/%d/prod.txt", year, day),
		fmt.Sprintf("inputs/%d/prod.txt", day),
	}
	for _, p := range paths {
		if _, err := os.Stat(p); err == nil {
			return p
		}
	}
	return ""
}
