package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
)

var regexpMul = "mul(d*,d*)"

func read03File(path string) (string, error) {
	dat, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	return string(dat), nil
}

func getMuls(input string) []string {
	pattern := regexp.MustCompile(`mul\(\d+,\d+\)`)
	return pattern.FindAllString(input, -1)
}

func stripDontParts(input string) string {
	re := regexp.MustCompile(`don\'t\(\).*?do\(\)`)
	for re.Match([]byte(input)) {
		input = re.ReplaceAllString(input, "")
	}
	return input
}

type Stack struct {
	depth int
	data  []string
}

func (s *Stack) Push(v string) {
	if s.depth == 0 {
		s.data = append(s.data, v)
	}
}

func (s *Stack) Pop() string {
	if len(s.data) == 0 {
		return ""
	}
	v := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	return v
}

func processOps(ops []operation) int {
	sum := 0
	for _, op := range ops {
		r := op.mult()
		sum += r
	}
	return sum
}

type operation struct {
	x, y int
}

func (o operation) mult() int {
	return o.x * o.y
}

func findEnabledMultOps(s string) []string {
	ops := []string{}

	rxs := map[string]*regexp.Regexp{
		"do":   regexp.MustCompile(`do\(\)`),
		"dont": regexp.MustCompile(`don't\(\)`),
		"mul":  regexp.MustCompile(`mul\(\d{1,3},\d{1,3}\)`),
	}

	mulOn := true
	for {
		var firstRxName, firstMatch string
		minIndex := math.MaxInt

		for name, rx := range rxs {
			loc := rx.FindStringIndex(s)
			if loc != nil && loc[0] < minIndex {
				minIndex = loc[0]
				firstRxName = name
				firstMatch = s[loc[0]:loc[1]]
			}
		}

		if firstRxName == "" {
			break
		}

		switch firstRxName {
		case "do":
			mulOn = true
		case "dont":
			mulOn = false
		case "mul":
			if mulOn {
				ops = append(ops, firstMatch)
			}
		}

		s = s[minIndex+len(firstMatch):]
	}

	return ops
}

func multiply(a, b int) int {
	return a * b
}

func sumValues(values []string) int {
	sum := 0
	for _, val := range values {
		var a, b int
		_, err := fmt.Sscanf(val, "mul(%d,%d)", &a, &b)
		if err != nil {
			panic(err)
		}
		sum += multiply(a, b)
	}
	return sum
}

func getSum(input []string) int {
	sum := 0
	for _, mul := range input {
		var a, b int
		_, err := fmt.Sscanf(mul, "mul(%d,%d)", &a, &b)
		if err != nil {
			panic(err)
		}
		sum += multiply(a, b)
	}
	return sum
}
