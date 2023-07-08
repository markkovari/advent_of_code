package days

import (
	"io/ioutil"
	"os"
	"path"
	"strings"

	"github.com/spf13/cast"
)

func ReadTextsOfDay(ofDay string) (string, string, error) {
	currentDir, err := os.Getwd()
	if err != nil {
		return "", "", err
	}

	examplePath := path.Join(currentDir, "..", "inputs", ofDay, "example.data")
	prodPath := path.Join(currentDir, "..", "inputs", ofDay, "prod.data")
	example, err := ioutil.ReadFile(examplePath)
	if err != nil {
		return "", "", err
	}
	prod, err := ioutil.ReadFile(prodPath)
	if err != nil {
		return "", "", err
	}
	return string(example), string(prod), nil
}

func Day1First(isProd bool) (int, error) {
	example, prod, err := ReadTextsOfDay("1")
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
func Day1Second(isProd bool) (int, error) {
	example, prod, err := ReadTextsOfDay("1")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part2(input), nil
}

func part1(input string) int {
	digits := parseInput(input)
	var sum int
	for i := 0; i < len(digits); i++ {
		current := digits[i]
		next := digits[(i+1)%len(digits)]
		if current == next {
			sum += digits[i]
		}
	}
	return sum
}

func part2(input string) int {
	digits := parseInput(input)
	var sum int
	offset := len(digits) / 2
	for i := 0; i < len(digits); i++ {
		if digits[i] == digits[(i+offset)%len(digits)] {
			sum += digits[i]
		}
	}
	return sum
}

func parseInput(input string) (ans []int) {
	for _, num := range strings.Split(input, "") {
		ans = append(ans, cast.ToInt(num))
	}
	return ans
}
