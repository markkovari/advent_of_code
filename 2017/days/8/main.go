package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day8First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("8")
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

func Day8Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("8")
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

	instructions, registers := parseInput(input)
	evaluateCommands(instructions, registers)
	max := 0
	for _, v := range registers {
		if v > max {
			max = v
		}
	}
	return max
}

func part2(input string) int {
	heighestRegisterValue := 0
	instructions, registers := parseInput(input)
	for _, instruction := range instructions {
		if evaluateCondition(instruction.Condition, registers) {
			switch instruction.Command {
			case "inc":
				registers[instruction.Register] += instruction.Amount
			case "dec":
				registers[instruction.Register] -= instruction.Amount
			}
			if registers[instruction.Register] > heighestRegisterValue {
				heighestRegisterValue = registers[instruction.Register]
			}
		}
	}
	return heighestRegisterValue

}

type Comperator string
type Command string

type Condition struct {
	Register   string
	Amount     int
	Comperator Comperator
}

type ConditionalInstruction struct {
	Register  string
	Condition Condition
	Command   Command
	Amount    int
}

func evaluateCondition(condition Condition, registers map[string]int) bool {

	register := condition.Register
	amount := condition.Amount
	comperator := condition.Comperator

	switch comperator {
	case ">":
		return registers[register] > amount
	case "<":
		return registers[register] < amount
	case ">=":
		return registers[register] >= amount
	case "<=":
		return registers[register] <= amount
	case "==":
		return registers[register] == amount
	case "!=":
		return registers[register] != amount
	default:
		error := fmt.Errorf("unknown comperator %v", comperator)
		fmt.Println(error)
		panic("unknown comperator")
	}
}

func evaluateCommands(instructions []ConditionalInstruction, registers map[string]int) {
	for _, instruction := range instructions {
		if evaluateCondition(instruction.Condition, registers) {
			switch instruction.Command {
			case "inc":
				registers[instruction.Register] += instruction.Amount
			case "dec":
				registers[instruction.Register] -= instruction.Amount
			}
		}
	}
}

func parseInput(input string) ([]ConditionalInstruction, map[string]int) {
	lines := strings.Split(input, "\n")
	instructions := make([]ConditionalInstruction, 0)
	registers := make(map[string]int, len(lines))
	for _, line := range lines {
		parts := strings.Fields(line)
		registers[parts[0]] = 0
		condition := Condition{Register: parts[4], Comperator: Comperator(parts[5]), Amount: cast.ToInt(parts[6])}
		instructions = append(instructions, ConditionalInstruction{
			Register:  parts[0],
			Condition: condition,
			Command:   Command(parts[1]),
			Amount:    cast.ToInt(parts[2]),
		})

	}
	return instructions, registers
}
