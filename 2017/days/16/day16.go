package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day16 struct{}

func (d Day16) Part1(input string) string {
	return permPromenade(input, 1)
}

func (d Day16) Part2(input string) string {
	return permPromenade(input, 1_000_000_000)
}

func permPromenade(input string, rounds int) string {
	steps := strings.Split(strings.TrimSpace(input), ",")
	programs := make([]byte, 16)
	for i := 0; i < 16; i++ {
		programs[i] = byte('a' + i)
	}

	seenStateToIndex := map[string]int{}
	for i := 0; i < rounds; i++ {
		for _, step := range steps {
			switch step[0] {
			case 's':
				var n int
				fmt.Sscanf(step, "s%d", &n)
				newProgs := append(programs[16-n:], programs[:16-n]...)
				copy(programs, newProgs)
			case 'x':
				var a, b int
				fmt.Sscanf(step, "x%d/%d", &a, &b)
				programs[a], programs[b] = programs[b], programs[a]
			case 'p':
				a, b := step[1], step[3]
				ia, ib := -1, -1
				for j, p := range programs {
					if p == a { ia = j }
					if p == b { ib = j }
				}
				programs[ia], programs[ib] = programs[ib], programs[ia]
			}
		}
		
		state := string(programs)
		if firstSeen, ok := seenStateToIndex[state]; ok {
			cycleLen := i - firstSeen
			remaining := (rounds - 1 - i) % cycleLen
			for j := 0; j < remaining; j++ {
				// Run through the cycle once more to reach final state
				for _, step := range steps {
					switch step[0] {
					case 's':
						var n int
						fmt.Sscanf(step, "s%d", &n)
						newProgs := append(programs[16-n:], programs[:16-n]...)
						copy(programs, newProgs)
					case 'x':
						var a, b int
						fmt.Sscanf(step, "x%d/%d", &a, &b)
						programs[a], programs[b] = programs[b], programs[a]
					case 'p':
						a, b := step[1], step[3]
						ia, ib := -1, -1
						for j, p := range programs {
							if p == a { ia = j }
							if p == b { ib = j }
						}
						programs[ia], programs[ib] = programs[ib], programs[ia]
					}
				}
			}
			return string(programs)
		}
		seenStateToIndex[state] = i
	}
	return string(programs)
}

func main() { common.Run(2017, 16, Day16{}) }
