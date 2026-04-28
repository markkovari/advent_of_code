package day20

import (
	"fmt"
	"strings"
)

type Solver struct{}

type SolverParticle struct {
	x, y, z, vx, vy, vz, ax, ay, az int64
	destroyed                       bool
}

func (d Solver) Part1(input string) string {
	particles := d.parse(input)
	minIdx := 0
	minAcc := int64(1 << 62)
	for i, p := range particles {
		acc := d.abs(p.ax) + d.abs(p.ay) + d.abs(p.az)
		if acc < minAcc {
			minAcc = acc
			minIdx = i
		}
	}
	return fmt.Sprintf("%d", minIdx)
}

func (d Solver) Part2(input string) string {
	particles := d.parse(input)
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
		if !p.destroyed {
			count++
		}
	}
	return fmt.Sprintf("%d", count)
}

func (d Solver) parse(input string) []SolverParticle {
	var particles []SolverParticle
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		var p SolverParticle
		fmt.Sscanf(line, "p=<%d,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>",
			&p.x, &p.y, &p.z, &p.vx, &p.vy, &p.vz, &p.ax, &p.ay, &p.az)
		particles = append(particles, p)
	}
	return particles
}

func (d Solver) abs(i int64) int64 {
	if i < 0 {
		return -i
	}
	return i
}
