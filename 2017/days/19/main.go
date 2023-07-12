package days

import (
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day19First(isProd bool) (string, error) {
	example, prod, err := utils.ReadTextsOfDay("19")
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

func Day19Second(isProd bool) (int, error) {
	example, prod, err := utils.ReadTextsOfDay("19")
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

func part1(input string) string {
	diagram := parseInput(input)
	path := make([]byte, 10)
	goSouth(1, strings.IndexRune(diagram[0], '|'), diagram, &path)
	return string(path)
}

func part2(input string) int {
	return steps
}

var steps int = 1

func goCommon(i, j *int, id, jd, is, js int, diagram []string, path *[]byte) {
	var done bool
	for ; *i >= 0 && *i < len(diagram) && *j >= 0 && *j < len(diagram[*i]) && !done; *i, *j = *i+id, *j+jd {
		steps++
		b := diagram[*i][*j]
		switch b {
		case ' ':
			*i += is
			*j += js
			done = true
		case '|':
		case '+':
		case '-':
		default:
			*path = append(*path, b)
		}
	}
	steps--
}

func goSouth(i, j int, diagram []string, path *[]byte) {
	goCommon(&i, &j, 1, 0, -2, 0, diagram, path)
	goWestEast(i, j, diagram, path)
}

func goNorth(i, j int, diagram []string, path *[]byte) {
	goCommon(&i, &j, -1, 0, 2, 0, diagram, path)
	goWestEast(i, j, diagram, path)
}
func goWest(i, j int, diagram []string, path *[]byte) {
	goCommon(&i, &j, 0, 1, 0, -2, diagram, path)
	goSouthNorth(i, j, diagram, path)
}
func goEast(i, j int, diagram []string, path *[]byte) {
	goCommon(&i, &j, 0, -1, 0, 2, diagram, path)
	goSouthNorth(i, j, diagram, path)
}

func goWestEast(i, j int, diagram []string, path *[]byte) {
	if j+1 < len(diagram[i]) && diagram[i][j+1] != ' ' {
		goWest(i, j+1, diagram, path)
	} else if j > 0 && diagram[i][j-1] != ' ' {
		goEast(i, j-1, diagram, path)
	}
}

func goSouthNorth(i, j int, diagram []string, path *[]byte) {
	if i+1 < len(diagram) && diagram[i+1][j] != ' ' {
		goSouth(i+1, j, diagram, path)
	} else if i > 0 && diagram[i-1][j] != ' ' {
		goNorth(i-1, j, diagram, path)
	}
}

func parseInput(content string) []string {
	content = strings.Trim(content, "\n")
	bytes := []byte(content)
	strs := strings.Split(string(bytes), "\n")
	return strs
}
