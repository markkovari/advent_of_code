package main

import (
	"fmt"
	"strings"

	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day20 struct{}

type Particle struct {
	x, y, z, vx, vy, vz, ax, ay, az int64
	destroyed                       bool
}

func (d Day20) Part1(input string) string {
	particles := parse(input)
	minIdx := 0
	minAcc := int64(1<<62)
	for i, p := range particles {
		acc := abs(p.ax) + abs(p.ay) + abs(p.az)
		if acc < minAcc {
			minAcc = acc
			minIdx = i
		}
	}
	return fmt.Sprintf("%d", minIdx)
}

func (d Day20) Part2(input string) string {
	particles := parse(input)
	for i := 0; i < 1000; i++ {
		positions := make(map[[3]int64][]int)
		for idx, p := range particles {
			if !p.destroyed {
				pos := [3]int64{p.x, p.y, p.z}
				positions[pos] = append(positions[pos], idx)
			}
		}
		for _, ids := range positions {
			if len(ids) > 1 {
				for _, id := range ids {
					particles[id].destroyed = true
				}
			}
		}
		for i := range particles {
			if !particles[i].destroyed {
				p := &particles[i]
				p.vx += p.ax
				p.vy += p.ay
				p.vz += p.az
				p.x += p.vx
				p.y += p.vy
				p.z += p.vz
			}
		}
	}
	count := 0
	for _, p := range particles {
		if !p.destroyed { count++ }
	}
	return fmt.Sprintf("%d", count)
}

func parse(input string) []Particle {
	var particles []Particle
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var p Particle
		fmt.Sscanf(line, "p=<%d,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>",
			&p.x, &p.y, &p.z, &p.vx, &p.vy, &p.vz, &p.ax, &p.ay, &p.az)
		particles = append(particles, p)
	}
	return particles
}

func abs(i int64) int64 {
	if i < 0 { return -i }
	return i
}

func main() { common.Run(2017, 20, Day20{}) }
