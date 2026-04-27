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
	case 1:
		solver = Day01{}
	case 2:
		solver = Day02{}
	case 3:
		solver = Day03{}
	case 4:
		solver = Day04{}
	case 5:
		solver = Day05{}
	case 6:
		solver = Day06{}
	case 7:
		solver = Day07{}
	case 8:
		solver = Day08{}
	case 9:
		solver = Day09{}
	case 10:
		solver = Day10{}
	case 11:
		solver = Day11{}
	case 12:
		solver = Day12{}
	case 13:
		solver = Day13{}
	case 14:
		solver = Day14{}
	case 15:
		solver = Day15{}
	case 16:
		solver = Day16{}
	case 17:
		solver = Day17{}
	case 18:
		solver = Day18{}
	case 19:
		solver = Day19{}
	case 20:
		solver = Day20{}
	case 21:
		solver = Day21{}
	case 22:
		solver = Day22{}
	case 23:
		solver = Day23{}
	case 24:
		solver = Day24{}
	case 25:
		solver = Day25{}
	default:
		fmt.Printf("Day %d not implemented\n", day)
		os.Exit(1)
	}

	common.Run(2017, day, solver)
}
