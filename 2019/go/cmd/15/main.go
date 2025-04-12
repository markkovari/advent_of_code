package main

import (
	"os"
	"strconv"
	"strings"
)

func readCommands() ([]int, error) {

	content, err := os.ReadFile("cmd/15/input.data")
	if err != nil {
		return []int{}, err
	}
	var numbers []int

	for _, value := range strings.Split(string(content), ",") {
		asNum, err := strconv.Atoi(value)

		if err == nil {
			numbers = append(numbers, asNum)
		}
	}

	return numbers, nil

}
func main() {

	nums, err := readCommands()
	if err != nil {
		println("was not able to read")
	}
	for _, num := range nums {
		println(num)
	}
}
