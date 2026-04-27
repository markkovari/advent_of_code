package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Please provide a day number")
		os.Exit(1)
	}

	day, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Printf("Invalid day: %s\n", os.Args[1])
		os.Exit(1)
	}

	var solver common.Solver

	switch day {
	case 15:
		solver = Day15{}
	default:
		fmt.Printf("Day %d not implemented\n", day)
		os.Exit(1)
	}

	common.Run(2019, day, solver)
}
