package days

import (
	"math"

	utils "github.com/markkovari/advent_of_code/2017/utils"

	"github.com/spf13/cast"
)

func Day3First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("3")
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
func Day3Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("3")
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
	num := parseInput(input)
	return calculateDistance(num)
}
func part2(input string) int {
	num := parseInput(input)
	return calculateStessedDistance(num)
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}
func calculateStessedDistance(inputNum int) int {

	directions := [][2]int{
		{0, 1},  // right
		{-1, 0}, // north
		{0, -1}, // west
		{1, 0},  // south
	}

	allNeighborDiffs := [][2]int{
		{-1, -1},
		{-1, 0},
		{-1, 1},
		{0, -1},
		{0, 1},
		{1, -1},
		{1, 0},
		{1, 1},
	}

	coordsToNeighborSum := map[[2]int]int{
		{0, 0}: 1,
		{0, 1}: 1,
	}

	directionIndex := 0
	row, col := 0, 1
	for len(coordsToNeighborSum) < inputNum {
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

		if sum > inputNum {
			return sum
		}

		coordsToNeighborSum[next] = sum
	}

	panic("should not reach here")
}

func calculateDistance(n int) int {
	if n == 1 {
		return 0
	}

	// Calculate ring and side lengths
	ring := int(math.Ceil((math.Sqrt(float64(n)) - 1) / 2))
	side := 2*ring + 1
	maxNum := side * side

	sideLen := side - 1
	midPoints := make([]int, 4)

	// Calculate midpoints of each side
	for i := 0; i < 4; i++ {
		midPoints[i] = maxNum - sideLen/2 - i*sideLen
	}

	// Find smallest distance to any midpoint
	minDist := abs(n - midPoints[0])
	for _, midPoint := range midPoints {
		dist := abs(n - midPoint)
		if dist < minDist {
			minDist = dist
		}
	}

	return ring + minDist
}

func parseInput(input string) (data int) {
	return cast.ToInt(input)
}
