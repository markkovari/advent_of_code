package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day01 struct{}

func (d Day01) Part1(input string) string {
	left, right := parse(input)
	sort.Slice(left, func(i, j int) bool { return left[i] < left[j] })
	sort.Slice(right, func(i, j int) bool { return right[i] < right[j] })

	var sum int64
	for i := range left {
		sum += int64(math.Abs(float64(right[i] - left[i])))
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day01) Part2(input string) string {
	left, right := parse(input)
	similarity := make(map[int64]int)
	for _, n := range right {
		similarity[n]++
	}

	var sum int64
	for _, n := range left {
		sum += n * int64(similarity[n])
	}
	return fmt.Sprintf("%d", sum)
}

func parse(input string) ([]int64, []int64) {
	left, right := make([]int64, 0), make([]int64, 0)
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		parts := strings.Fields(line)
		l, _ := strconv.ParseInt(parts[0], 10, 64)
		r, _ := strconv.ParseInt(parts[1], 10, 64)
		left = append(left, l)
		right = append(right, r)
	}
	return left, right
}

func main() {
	common.Run(2024, 1, Day01{})
}
