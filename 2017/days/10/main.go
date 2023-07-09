package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day10First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("10")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part1(input, 256), nil
}

func Day10Second(isProd bool) (string, error) {
	example, prod, err := utils.ReadTextsOfDay("10")
	if err != nil {
		return "", err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}
	return part2(input), nil
}

func part1(input string, listLength int) int {
	lengths := parseInput(input)

	nums := make([]int, listLength)
	for i := range nums {
		nums[i] = i
	}
	var position, skipSize int
	for _, length := range lengths {
		if length > 0 {
			nums = reverse(nums, position, position+length-1)
		}
		position += skipSize + length
		position %= len(nums)
		skipSize++
	}

	return nums[0] * nums[1]
}

func part2(input string) string {
	lengths := parseInputASCII(input)
	nums := make([]int, 256)
	for i := range nums {
		nums[i] = i
	}
	var position, skipSize int

	for round := 0; round < 64; round++ {
		for _, length := range lengths {
			if length > 0 {
				nums = reverse(nums, position, position+length-1)
			}
			position += skipSize + length
			position %= len(nums)
			skipSize++
		}
	}

	var denseHash []int
	for i := 0; i < 16; i++ {
		var xord int
		for j := i * 16; j < (i+1)*16; j++ {
			xord ^= nums[j]
		}
		denseHash = append(denseHash, xord)
	}

	var hexdHash string
	for _, dense := range denseHash {
		// use %x to get hexadecimal version & 02 ensures leading 0 if needed
		hexdHash += fmt.Sprintf("%02x", dense)
	}

	return hexdHash
}

func parseInput(input string) (ans []int) {
	nums := strings.Split(input, ",")
	for _, num := range nums {
		ans = append(ans, cast.ToInt(num))
	}
	return ans
}

func reverse(nums []int, left, right int) []int {
	right %= len(nums)
	if right < left {
		right += len(nums)
	}

	for left < right {
		leftModded := left % len(nums)
		rightModded := right % len(nums)
		nums[leftModded], nums[rightModded] = nums[rightModded], nums[leftModded]
		left++
		right--
	}

	return nums
}

func parseInputASCII(input string) (ans []int) {
	for _, char := range input {
		ans = append(ans, int(char))
	}
	ans = append(ans, 17, 31, 73, 47, 23)
	return ans
}
