package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Day14 struct{}

func (d Day14) Part1(input string) string {
	grid := d.buildDisk(input)
	count := 0
	for r := 0; r < 128; r++ {
		for c := 0; c < 128; c++ {
			if grid[r][c] {
				count++
			}
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day14) Part2(input string) string {
	grid := d.buildDisk(input)
	count := 0
	dirs := [4][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for r := 0; r < 128; r++ {
		for c := 0; c < 128; c++ {
			if grid[r][c] {
				count++
				queue := [][2]int{{r, c}}
				grid[r][c] = false
				for len(queue) > 0 {
					curr := queue[0]
					queue = queue[1:]
					for _, d := range dirs {
						nr, nc := curr[0]+d[0], curr[1]+d[1]
						if nr >= 0 && nr < 128 && nc >= 0 && nc < 128 && grid[nr][nc] {
							grid[nr][nc] = false
							queue = append(queue, [2]int{nr, nc})
						}
					}
				}
			}
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Day14) buildDisk(input string) [][]bool {
	disk := make([][]bool, 128)
	key := strings.TrimSpace(input)
	for i := 0; i < 128; i++ {
		hash := d.knotHash(fmt.Sprintf("%s-%d", key, i))
		disk[i] = make([]bool, 128)
		for j, char := range hash {
			val, _ := strconv.ParseInt(string(char), 16, 64)
			for b := 0; b < 4; b++ {
				if (val & (1 << (3 - b))) != 0 {
					disk[i][j*4+b] = true
				}
			}
		}
	}
	return disk
}

func (d Day14) knotHash(input string) string {
	lengths := make([]int, 0)
	for _, c := range input {
		lengths = append(lengths, int(c))
	}
	lengths = append(lengths, 17, 31, 73, 47, 23)

	nums := make([]int, 256)
	for i := range nums {
		nums[i] = i
	}
	pos, skip := 0, 0
	for round := 0; round < 64; round++ {
		for _, l := range lengths {
			for i := 0; i < l/2; i++ {
				a, b := (pos+i)%256, (pos+l-1-i)%256
				nums[a], nums[b] = nums[b], nums[a]
			}
			pos = (pos + l + skip) % 256
			skip++
		}
	}
	var res string
	for i := 0; i < 16; i++ {
		block := 0
		for j := 0; j < 16; j++ {
			block ^= nums[i*16+j]
		}
		res += fmt.Sprintf("%02x", block)
	}
	return res
}
