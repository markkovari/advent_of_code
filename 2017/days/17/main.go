package days

import (
	"log"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day17First(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("17")
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

func Day17Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("17")
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
	return spinlock(input, 1)
}

func part2(input string) int {
	return spinlock(input, 2)
}

type llNode struct {
	value int
	next  *llNode
}

func spinlock(input string, part int) int {
	steps := cast.ToInt(input)

	lastNumToAdd := 2017
	if part == 2 {
		lastNumToAdd = 50000000
	}

	current := &llNode{value: 0}
	current.next = current
	for i := 1; i <= lastNumToAdd; i++ {
		for j := 0; j < steps; j++ {
			current = current.next
		}

		saveNext := current.next
		current.next = &llNode{
			value: i,
			next:  saveNext,
		}
		current = current.next

		if i%1000000 == 0 {
			log.Println(i, "steps done")
		}
	}

	valueToFind := 2017
	if part == 2 {
		valueToFind = 0
	}
	for current.value != valueToFind {
		current = current.next
	}

	return current.next.value
}
