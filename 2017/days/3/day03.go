package main

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day03 struct{}

func (d Day03) Part1(input string) string {
	num, _ := strconv.Atoi(strings.TrimSpace(input))
	return fmt.Sprintf("%d", calculateDistance(num))
}

func (d Day03) Part2(input string) string {
	num, _ := strconv.Atoi(strings.TrimSpace(input))
	return fmt.Sprintf("%d", calculateStressedDistance(num))
}

func abs(x int) int {
	if x < 0 { return -x }
	return x
}

func calculateDistance(n int) int {
	if n == 1 { return 0 }
	ring := int(math.Ceil((math.Sqrt(float64(n)) - 1) / 2))
	side := 2*ring + 1
	maxNum := side * side
	sideLen := side - 1
	midPoints := make([]int, 4)
	for i := 0; i < 4; i++ {
		midPoints[i] = maxNum - sideLen/2 - i*sideLen
	}
	minDist := abs(n - midPoints[0])
	for _, midPoint := range midPoints {
		dist := abs(n - midPoint)
		if dist < minDist { minDist = dist }
	}
	return ring + minDist
}

func calculateStressedDistance(inputNum int) int {
	directions := [][2]int{{0, 1}, {-1, 0}, {0, -1}, {1, 0}}
	allNeighborDiffs := [][2]int{{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}}
	coordsToNeighborSum := map[[2]int]int{{0, 0}: 1, {0, 1}: 1}
	directionIndex := 0
	row, col := 0, 1
	for {
		leftDiff := directions[(directionIndex+1)%4]
		leftCoord := [2]int{row + leftDiff[0], col + leftDiff[1]}
		if _, ok := coordsToNeighborSum[leftCoord]; !ok {
			directionIndex = (directionIndex + 1) % 4
		}
		diff := directions[directionIndex]
		row += diff[0]
		col += diff[1]
		next := [2]int{row, col}
		var sum int
		for _, d := range allNeighborDiffs {
			sum += coordsToNeighborSum[[2]int{row + d[0], col + d[1]}]
		}
		if sum > inputNum { return sum }
		coordsToNeighborSum[next] = sum
	}
}

func main() {
	common.Run(2017, 3, Day03{})
}
