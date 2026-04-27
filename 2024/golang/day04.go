package main

import (
	"fmt"
	"strings"
)

type Day04 struct{}

func (d Day04) Part1(input string) string {
	grid := d.parse(input)
	count := 0
	word := "XMAS"
	rows := len(grid)
	cols := len(grid[0])

	dirs := [][2]int{
		{0, 1}, {0, -1}, {1, 0}, {-1, 0},
		{1, 1}, {1, -1}, {-1, 1}, {-1, -1},
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			for _, dir := range dirs {
				if d.check(grid, r, c, dir, word) {
					count++
				}
			}
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day04) Part2(input string) string {
	grid := d.parse(input)
	count := 0
	rows := len(grid)
	cols := len(grid[0])

	for r := 1; r < rows-1; r++ {
		for c := 1; c < cols-1; c++ {
			if grid[r][c] == 'A' {
				// M.M
				// .A.
				// S.S
				tl := grid[r-1][c-1]
				tr := grid[r-1][c+1]
				bl := grid[r+1][c-1]
				br := grid[r+1][c+1]

				if ((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')) &&
					((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M')) {
					count++
				}
			}
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day04) parse(input string) [][]rune {
	var grid [][]rune
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		grid = append(grid, []rune(line))
	}
	return grid
}

func (d Day04) check(grid [][]rune, r, c int, dir [2]int, word string) bool {
	for i := 0; i < len(word); i++ {
		nr, nc := r+dir[0]*i, c+dir[1]*i
		if nr < 0 || nr >= len(grid) || nc < 0 || nc >= len(grid[0]) || grid[nr][nc] != rune(word[i]) {
			return false
		}
	}
	return true
}
