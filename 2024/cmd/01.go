package main

import (
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func readFile(path string) ([]int64, []int64, error) {
	dat, err := os.ReadFile(path)
	if err != nil {
		return nil, nil, err
	}
	lines := strings.Split(string(dat), "\n")
	left := make([]int64, 0)
	right := make([]int64, 0)
	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		leftAsNumber, err := strconv.ParseInt(parts[0], 10, 64)
		if err != nil {
			continue
		}
		rightAsNumber, err := strconv.ParseInt(parts[1], 10, 64)
		if err != nil {
			continue
		}
		left = append(left, int64(leftAsNumber))
		right = append(right, int64(rightAsNumber))
	}
	return left, right, nil
}

func sortInput(left, right []int64) ([]int64, []int64) {
	slices.Sort(left)
	slices.Sort(right)
	return left, right
}

func getDistancesWithSimilarity(left, right []int64) []int64 {
	similarity := getSimilarity(right)
	distances := make([]int64, 0)
	for i := 0; i < len(left); i++ {
		value := left[i] * int64(similarity[left[i]])
		distances = append(distances, value)
	}
	return distances
}

func getDistances(left, right []int64) []int64 {
	distances := make([]int64, 0)
	for i := 0; i < len(left); i++ {
		d := int64(math.Abs(float64(right[i] - left[i])))
		distances = append(distances, d)
	}
	return distances
}

func sumDistances(distances []int64) int {
	var sum int
	for _, distance := range distances {
		sum += int(distance)
	}
	return sum
}

func getSimilarity(numbers []int64) map[int64]int {
	similarity := make(map[int64]int)
	for _, number := range numbers {
		if _, ok := similarity[number]; ok {
			similarity[number]++
		} else {
			similarity[number] = 1
		}
	}

	return similarity
}
