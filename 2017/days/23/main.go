package days

import (
	"strconv"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

type computer struct {
	instructions [][]string
	pointer      int
	registers    map[string]int
	output       []int
}

func newComputerFromInput(input string) *computer {
	comp := &computer{registers: map[string]int{}}
	for _, line := range strings.Split(input, "\n") {
		comp.instructions = append(comp.instructions, strings.Split(line, " "))
	}

	return comp
}

func (c *computer) countMulsRun() (mulsRun int) {
	for c.pointer < len(c.instructions) {
		inst := c.instructions[c.pointer]
		valX := inst[1]
		var valFromY int
		if val, err := strconv.Atoi(inst[2]); err != nil {
			valFromY = c.registers[inst[2]]
		} else {
			valFromY = val
		}

		switch inst[0] {
		case "set":
			c.registers[valX] = valFromY
			c.pointer++
		case "sub":
			c.registers[valX] -= valFromY
			c.pointer++
		case "mul":
			c.registers[valX] *= valFromY
			c.pointer++
			mulsRun++
		case "jnz":
			var parsedX int
			if num, err := strconv.Atoi(valX); err != nil {
				parsedX = c.registers[valX]
			} else {
				parsedX = num
			}
			if parsedX != 0 {
				c.pointer += valFromY
			} else {
				c.pointer++
			}
		default:
			panic("unhandled operator " + inst[0])
		}
	}
	return mulsRun
}

func Day23First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("23")
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

func Day23Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("23")
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
	comp := newComputerFromInput(input)
	return comp.countMulsRun()
}

func part2(input string) int {
	b := 81
	c := 81

	b = b*100 + 100000
	c = b + 17000
	var h int
	for {
		f := 1
		for d := 2; d*d <= b; d++ {
			if b%d == 0 {
				f = 0
				break
			}
		}

		if f == 0 {
			h++
		}
		if b == c {
			break
		}
		b += 17
	}

	return h
}
