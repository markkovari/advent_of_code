package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day6First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("6")
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

func Day6Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("6")
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
	return memoryReallocation(input, 1)
}

func part2(input string) int {
	return memoryReallocation(input, 2)
}

func parse16Input(input string) (ans [16]int) {
	nums := strings.Split(input, "\t")
	for i, num := range nums {
		ans[i] = cast.ToInt(num)
	}
	return ans
}

func memoryReallocation(input string, part int) int {
	banks := parse16Input(input)

	seenBanks := map[[16]int]int{banks: 0}
	var cycles int
	for {
		// find largest bank
		var index, maxVal int
		for i, val := range banks {
			if val > maxVal {
				index = i
				maxVal = val
			}
		}

		// run a cycle
		blocksToDistribute := banks[index]
		banks[index] = 0
		// unoptimized but works just fine
		for i := index + 1; blocksToDistribute > 0; i++ {
			if blocksToDistribute == 0 {
				break
			}
			banks[i%16]++
			blocksToDistribute--
		}
		cycles++

		// check if this set of banks has been seen before, if so return here
		if val, ok := seenBanks[banks]; ok {
			if part == 1 {
				return cycles
			}
			// for part 2 take the difference with cycles when it was last seen
			return cycles - val
		}
		// set the number of cycles that correspond to this state of banks
		seenBanks[banks] = cycles
	}

	panic("should resolve in for loop")
}
