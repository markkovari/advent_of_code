package day17

import (
	"fmt"
	"strconv"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	steps, _ := strconv.Atoi(strings.TrimSpace(input))
	buffer := []int{0}
	pos := 0
	for i := 1; i <= 2017; i++ {
		pos = (pos+steps)%len(buffer) + 1
		buffer = append(buffer[:pos], append([]int{i}, buffer[pos:]...)...)
	}
	return fmt.Sprintf("%d", buffer[pos+1])
}

func (d Solver) Part2(input string) string {
	steps, _ := strconv.Atoi(strings.TrimSpace(input))
	valAfterZero, pos := 0, 0
	for i := 1; i <= 50_000_000; i++ {
		pos = (pos+steps)%i + 1
		if pos == 1 {
			valAfterZero = i
		}
	}
	return fmt.Sprintf("%d", valAfterZero)
}
