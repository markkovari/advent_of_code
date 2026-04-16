package main

import (
	"fmt"
	"sort"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day04 struct{}

func (d Day04) Part1(input string) string {
	countValid := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		words := strings.Fields(line)
		frequency := make(map[string]int)
		valid := true
		for _, word := range words {
			frequency[word]++
			if frequency[word] > 1 {
				valid = false
				break
			}
		}
		if valid { countValid++ }
	}
	return fmt.Sprintf("%d", countValid)
}

func (d Day04) Part2(input string) string {
	countValid := 0
	for _, line := range strings.Split(input, "\n") {
		if line == "" { continue }
		words := strings.Fields(line)
		if !hasAnagram(words) {
			countValid++
		}
	}
	return fmt.Sprintf("%d", countValid)
}

func hasAnagram(words []string) bool {
	sortedWords := make([]string, len(words))
	for i, word := range words {
		chars := strings.Split(word, "")
		sort.Strings(chars)
		sortedWords[i] = strings.Join(chars, "")
	}
	for i := 0; i < len(sortedWords); i++ {
		for j := i + 1; j < len(sortedWords); j++ {
			if sortedWords[i] == sortedWords[j] {
				return true
			}
		}
	}
	return false
}

func main() {
	common.Run(2017, 4, Day04{})
}
