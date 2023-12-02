package main

import (
	"os"
	"strconv"

	"github.com/markkovari/aoc/internal"
)

var days = map[int]internal.AocDay{
	1: internal.AocDay{Solve: internal.FirstHandler},
}

func main() {
	args := os.Args[1:]
	if len(args) != 2 {
		print("Usage: aoc <day> \"true|false\"\n")
		os.Exit(1)
	}

	isProd, err := strconv.ParseBool(args[1])
	if err != nil {
		print("Usage: aoc <day> \"true|false\"\n")
		os.Exit(1)
	}

	atDay, err := strconv.ParseUint(args[0], 10, 8)
	if err != nil {
		print("Usage: aoc <day> \"true|false\"\n")
		os.Exit(1)
	}

	if day, ok := days[int(atDay)]; ok {
		first, second := day.Solve(int(atDay), isProd)
		print("First: ", first, "\n")
		print("Second: ", second, "\n")
		return
	}
}
