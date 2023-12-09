package internal

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type digit struct {
	value   int
	forward string
}

type digitWithPosition struct {
	position int
	digit
}

var (
	digits = []digit{
		{0, "zero"},
		{1, "one"},
		{2, "two"},
		{3, "three"},
		{4, "four"},
		{5, "five"},
		{6, "six"},
		{7, "seven"},
		{8, "eight"},
		{9, "nine"},
	}
)

func (d digit) getFirstPositionIn(value string) (int, int, bool) {
	firstWithLetters := strings.Index(value, d.forward)
	firstWithDigit := strings.Index(value, fmt.Sprint(d.value))
	if firstWithLetters == -1 && firstWithDigit == -1 {
		return -1, d.value, false
	}
	if firstWithLetters != -1 {
		return firstWithLetters, d.value, true
	}
	return firstWithDigit, d.value, true
}

func (d digit) getLastPositionIn(value string) *digitWithPosition {
	firstWithLetters := strings.LastIndex(value, d.forward)
	firstWithDigit := strings.LastIndex(value, fmt.Sprint(d.value))
	if firstWithLetters == -1 && firstWithDigit == -1 {
		return nil
	}
	if firstWithLetters != -1 {
		return &digitWithPosition{firstWithLetters, d}
	}
	return &digitWithPosition{firstWithDigit, d}
}

func getFirstDigit(digits []digitWithPosition) digit {
	d := digits[0]
	for _, digit := range digits {
		if digit.position < d.position {
			d = digit
		}
	}
	return digit{d.value, d.forward}
}

func getMax(values []int) int {
	max := values[0]
	for _, v := range values {
		if v > max {
			max = v
		}
	}
	return max
}

func readFile(index int, isData bool) []string {
	fileName := "inputs/" + strconv.Itoa(index)
	if isData {
		fileName += "/data"
	} else {
		fileName += "/example"
	}

	file, err := os.Open(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	lines := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	return lines
}

func findNumber(line []byte, start, end, move int, multiply byte, ch chan<- byte) error {
	for i := start; i != end; i += move {
		if line[i] >= 48 && line[i] <= 57 {
			ch <- (line[i] - 48) * multiply
			return nil
		}
	}

	return fmt.Errorf("unable to find number in: %s", line)
}

func part1(index int, isPord bool) int {
	lines := readFile(index, isPord)
	var total = 0
	fc := make(chan byte)
	lc := make(chan byte)
	for _, line := range lines {
		go func() {
			if err := findNumber([]byte(line), 0, len(line), 1, 10, fc); err != nil {
				panic(err)
			}
		}()
		go func() {
			if err := findNumber([]byte(line), len(line)-1, -1, -1, 1, lc); err != nil {
				panic(err)
			}
		}()

		total += int(<-fc + <-lc)
	}
	return total
}

func part2(index int, isPord bool) int {
	lines := readFile(index, isPord)
	var total = 0
	for _, line := range lines {
		first := _getFirstDigit(line)
		last := getLastDigit(line)
		println(line)
		println(first, last)
	}
	return total
}

func FirstHandler(index int, isProd bool) (int, int) {
	return 1, part2(index, isProd)
}
