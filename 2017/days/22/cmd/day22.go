package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day22 struct{}

type infectedState int
const ( clean infectedState = iota; weakened; infected; flagged )

func (d Day22) Part1(input string) string {
    state := newState(input)
    current := [2]int{len(strings.Split(input, "\n")) / 2, len(strings.Split(input, "\n")) / 2}
    dir, countBursts := 0, 0
    dirs := [][2]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

    for i := 0; i < 10000; i++ {
        if state[current] == infected {
            dir = (dir + 1) % 4
            state[current] = clean
        } else {
            dir = (dir + 3) % 4
            state[current] = infected
            countBursts++
        }
        current = [2]int{current[0] + dirs[dir][0], current[1] + dirs[dir][1]}
    }
    return fmt.Sprintf("%d", countBursts)
}

func (d Day22) Part2(input string) string {
    state := newState(input)
    current := [2]int{len(strings.Split(input, "\n")) / 2, len(strings.Split(input, "\n")) / 2}
    dir, countBursts := 0, 0
    dirs := [][2]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}

    for i := 0; i < 10000000; i++ {
        switch state[current] {
        case clean:
            dir = (dir + 3) % 4
            state[current] = weakened
        case weakened:
            state[current] = infected
            countBursts++
        case infected:
            dir = (dir + 1) % 4
            state[current] = flagged
        case flagged:
            dir = (dir + 2) % 4
            state[current] = clean
        }
        current = [2]int{current[0] + dirs[dir][0], current[1] + dirs[dir][1]}
    }
    return fmt.Sprintf("%d", countBursts)
}

func newState(input string) map[[2]int]infectedState {
	ans := map[[2]int]infectedState{}
	for r, line := range strings.Split(input, "\n") {
		for c, v := range line {
			if v == '#' { ans[[2]int{r, c}] = infected } else if v == '.' { ans[[2]int{r, c}] = clean }
		}
	}
	return ans
}

func main() { common.Run(2017, 22, Day22{}) }
