package internal

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strconv"
)

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

	return errors.New(fmt.Sprintf("unable to find number in: %s", line))
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

func FirstHandler(index int, isProd bool) (int, int) {

	return part1(index, isProd), 1
}
