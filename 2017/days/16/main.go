package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
	"github.com/spf13/cast"
)

func Day16First(isProd bool) (string, error) {
	example, prod, err := utils.ReadTextsOfDay("16")
	if err != nil {
		return "", err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = example
	}

	return part1(input), nil
}

func Day16Second(isProd bool) (string, error) {
	example, prod, err := utils.ReadTextsOfDay("16")
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

func part1(input string) string {
	return permPromenade(input, 1)
}

func part2(input string) string {
	return permPromenade(input, 2)
}

func permPromenade(input string, part int) string {
	steps := strings.Split(input, ",")

	var programs []string
	for i := 0; i < 16; i++ {
		programs = append(programs, string(rune('a'+i)))
	}

	rounds := 1
	if part == 2 {
		rounds = 1000000000
	}

	seenStateToIndex := map[string]int{}
	for i := 0; i < rounds; i++ {
		for _, step := range steps {
			switch step[0] {
			case 's':
				countToSpin := cast.ToInt(step[1:])
				fromEnd := programs[len(programs)-countToSpin:]
				fromFront := programs[:len(programs)-countToSpin]
				programs = append(fromEnd, fromFront...)
			case 'x':
				var index1, index2 int
				_, err := fmt.Sscanf(step, "x%d/%d", &index1, &index2)
				if err != nil {
					panic("error parsing an 'x' step " + err.Error())
				}
				programs[index1], programs[index2] = programs[index2], programs[index1]
			case 'p':
				var char1, char2 string
				_, err := fmt.Sscanf(step, "p%1s/%1s", &char1, &char2)
				if err != nil {
					panic("error parsing a 'p' step " + err.Error())
				}
				index1, index2 := -1, -1
				for i, v := range programs {
					if v == char1 {
						index1 = i
					} else if v == char2 {
						index2 = i
					}
				}
				programs[index1], programs[index2] = programs[index2], programs[index1]
			default:
				panic("not found step type " + string(step[0]))
			}
		}
		state := stringify(programs)
		if lastSeenIndex, ok := seenStateToIndex[state]; ok {
			diff := i - lastSeenIndex
			for i+diff < rounds {
				i += diff
			}
		}
		seenStateToIndex[state] = i
	}

	return stringify(programs)
}

func stringify(programs []string) string {
	var state string
	for _, v := range programs {
		state += v
	}
	return state
}
