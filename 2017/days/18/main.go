package days

import (
	"math"
	"strconv"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

var noInput = math.MinInt16

type computer struct {
	instructions [][]string
	pointer      int
	registers    map[string]int
	output       []int
}

func newComputerFromInput(input string, programID int) *computer {
	comp := &computer{registers: map[string]int{"p": programID}}
	for _, line := range strings.Split(input, "\n") {
		comp.instructions = append(comp.instructions, strings.Fields(line))
	}
	return comp
}

func (c *computer) step(inputNum int) {
	for {
		inst := c.instructions[c.pointer]
		valX := inst[1]
		var valY int
		if len(inst) == 3 && inst[2] != "" {
			if val, err := strconv.Atoi(inst[2]); err != nil {
				valY = c.registers[inst[2]]
			} else {
				valY = val
			}
		}

		switch inst[0] {
		case "snd":
			c.output = append(c.output, c.registers[valX])
			c.pointer++
		case "set":
			c.registers[valX] = valY
			c.pointer++
		case "add":
			c.registers[valX] += valY
			c.pointer++
		case "mul":
			c.registers[valX] *= valY
			c.pointer++
		case "mod":
			c.registers[valX] %= valY
			c.pointer++
		case "rcv":
			if inputNum == noInput {
				return
			}
			c.registers[valX] = inputNum
			inputNum = noInput
			c.pointer++
		case "jgz":
			var parsedX int
			if num, err := strconv.Atoi(valX); err != nil {
				parsedX = c.registers[valX]
			} else {
				parsedX = num
			}
			if parsedX > 0 {
				c.pointer += valY + len(c.instructions)
				c.pointer %= len(c.instructions)
			} else {
				c.pointer++
			}
		default:
			panic("unhandled operator " + inst[0])
		}
	}
}

func Day18First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("18")
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

func Day18Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("18")
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
	comp := newComputerFromInput(input, 0)
	comp.step(noInput)
	return comp.output[len(comp.output)-1]
}

func part2(input string) int {
	program0 := newComputerFromInput(input, 0)
	program1 := newComputerFromInput(input, 1)

	program0.step(noInput)
	program1.step(noInput)

	var sentFrom1 int

	for len(program0.output)+len(program1.output) > 0 {
		for len(program0.output) > 0 {
			v := program0.output[0]
			program0.output = program0.output[1:]
			program1.step(v)
		}
		for len(program1.output) > 0 {
			v := program1.output[0]
			program1.output = program1.output[1:]
			program0.step(v)
			sentFrom1++
		}
	}

	return sentFrom1
}
