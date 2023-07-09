package days

import (
	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day9First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("9")
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

func Day9Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("9")
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
	return streamProcessing(input, 1)
}

func part2(input string) int {
	return streamProcessing(input, 2)
}

func streamProcessing(input string, part int) int {
	var totalScore, garbageCount int
	var inGarbage bool
	var openCurlies int

	for i := 0; i < len(input); i++ {
		char := string(input[i])
		if inGarbage {
			switch char {
			case "!":
				i++
			case ">":
				inGarbage = false
			default:
				garbageCount++
			}
		} else {
			switch char {
			case "{":
				openCurlies++
			case "}":
				totalScore += openCurlies
				openCurlies--
			case "<":
				inGarbage = true
			}
		}
	}

	if part == 1 {
		return totalScore
	}
	return garbageCount
}
