package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day4First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("4")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part1(input), nil
}
func Day4Second(isProd bool) (int, error) {
	_, prod, err := utils.ReadTextsOfDay("4")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = `abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio`
	}
	return part2(input), nil
}

func part1(input string) int {
	lines := parseInput(input)
	countValid := 0
	for _, line := range lines {
		frequency := getFrequency(line, " ")
		valid := true
		for _, count := range frequency {
			if count > 1 {
				valid = false
				break
			}
		}

		if valid {
			countValid++
		}
	}
	return countValid
}

func part2(input string) int {
	lines := parseInput(input)
	countValid := 0
	for _, line := range lines {
		if !hasAnagram(line) {
			countValid++
		}
	}
	return countValid
}
func hasAnagram(line string) bool {
	words := strings.Split(line, " ")
	for i, word := range words {
		for j, word2 := range words {
			if i == j {
				continue
			}
			if isAnagram(word, word2) {
				return true
			}
		}
	}
	return false
}

func isAnagram(word1, word2 string) bool {
	if len(word1) != len(word2) {
		return false
	}
	frequency1 := getFrequency(word1, "")
	frequency2 := getFrequency(word2, "")
	for key, count := range frequency1 {
		if frequency2[key] != count {
			return false
		}
	}
	return true
}

func getFrequency(line string, splitBy string) map[string]int {
	frequency := make(map[string]int)
	words := strings.Split(line, splitBy)

	for _, word := range words {
		frequency[word]++
	}
	return frequency
}

func parseInput(input string) (lines []string) {
	return strings.Split(input, "\n")
}
