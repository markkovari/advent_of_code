package main

import (
	"fmt"
	"regexp"
	"strconv"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day03 struct{}

func (d Day03) Part1(input string) string {
	pattern := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	matches := pattern.FindAllStringSubmatch(input, -1)
	var sum int64
	for _, m := range matches {
		sum += toInt(m[1]) * toInt(m[2])
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day03) Part2(input string) string {
	pattern := regexp.MustCompile(`mul\((\d+),(\d+)\)|do\(\)|don't\(\)`)
	matches := pattern.FindAllStringSubmatch(input, -1)
	var sum int64
	enabled := true
	for _, m := range matches {
		switch m[0] {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled {
				sum += toInt(m[1]) * toInt(m[2])
			}
		}
	}
	return fmt.Sprintf("%d", sum)
}

func toInt(s string) int64 {
	val, _ := strconv.ParseInt(s, 10, 64)
	return val
}

func main() {
	common.Run(2024, 3, Day03{})
}
