package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/markkovari/advent_of_code/golang/common"
	"github.com/markkovari/advent_of_code/golang/2017/day01"
	"github.com/markkovari/advent_of_code/golang/2017/day02"
	"github.com/markkovari/advent_of_code/golang/2017/day03"
	"github.com/markkovari/advent_of_code/golang/2017/day04"
	"github.com/markkovari/advent_of_code/golang/2017/day05"
	"github.com/markkovari/advent_of_code/golang/2017/day06"
	"github.com/markkovari/advent_of_code/golang/2017/day07"
	"github.com/markkovari/advent_of_code/golang/2017/day08"
	"github.com/markkovari/advent_of_code/golang/2017/day09"
	"github.com/markkovari/advent_of_code/golang/2017/day10"
	"github.com/markkovari/advent_of_code/golang/2017/day11"
	"github.com/markkovari/advent_of_code/golang/2017/day12"
	"github.com/markkovari/advent_of_code/golang/2017/day13"
	"github.com/markkovari/advent_of_code/golang/2017/day14"
	"github.com/markkovari/advent_of_code/golang/2017/day15"
	"github.com/markkovari/advent_of_code/golang/2017/day16"
	"github.com/markkovari/advent_of_code/golang/2017/day17"
	"github.com/markkovari/advent_of_code/golang/2017/day18"
	"github.com/markkovari/advent_of_code/golang/2017/day19"
	"github.com/markkovari/advent_of_code/golang/2017/day20"
	"github.com/markkovari/advent_of_code/golang/2017/day21"
	"github.com/markkovari/advent_of_code/golang/2017/day22"
	"github.com/markkovari/advent_of_code/golang/2017/day23"
	"github.com/markkovari/advent_of_code/golang/2017/day24"
	"github.com/markkovari/advent_of_code/golang/2017/day25"
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
		solver = day01.Solver{}
	case 2:
		solver = day02.Solver{}
	case 3:
		solver = day03.Solver{}
	case 4:
		solver = day04.Solver{}
	case 5:
		solver = day05.Solver{}
	case 6:
		solver = day06.Solver{}
	case 7:
		solver = day07.Solver{}
	case 8:
		solver = day08.Solver{}
	case 9:
		solver = day09.Solver{}
	case 10:
		solver = day10.Solver{}
	case 11:
		solver = day11.Solver{}
	case 12:
		solver = day12.Solver{}
	case 13:
		solver = day13.Solver{}
	case 14:
		solver = day14.Solver{}
	case 15:
		solver = day15.Solver{}
	case 16:
		solver = day16.Solver{}
	case 17:
		solver = day17.Solver{}
	case 18:
		solver = day18.Solver{}
	case 19:
		solver = day19.Solver{}
	case 20:
		solver = day20.Solver{}
	case 21:
		solver = day21.Solver{}
	case 22:
		solver = day22.Solver{}
	case 23:
		solver = day23.Solver{}
	case 24:
		solver = day24.Solver{}
	case 25:
		solver = day25.Solver{}
	default:
		fmt.Printf("Day %d not implemented\n", day)
		os.Exit(1)
	}

	common.Run(2017, day, solver)
}
