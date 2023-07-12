package days

import (
	"fmt"
	"strings"

	utils "github.com/markkovari/advent_of_code/2017/utils"
)

func Day20First(isProd bool) (int, error) {
	_, prod, err := utils.ReadTextsOfDay("20")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = `p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
		p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>`
	}

	return part1(input), nil
}

func Day20Second(isProd bool) (int, error) {
	_, prod, err := utils.ReadTextsOfDay("20")
	if err != nil {
		return 0, err
	}
	var input string
	if isProd {
		input = prod
	} else {
		input = `p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>
		p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
		p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
		p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>`
	}
	return part2(input), nil
}

type position struct {
	x, y, z int64
}

type particle struct {
	pos                    position
	vx, vy, vz, ax, ay, az int64
	destroyed              bool
}

func part1(content string) int {
	particles := parse(strings.Split(content, "\n"))

	a_min := ^uint(0)
	var min_idx int
	for idx, p := range particles {
		a := abs(p.ax) + abs(p.ay) + abs(p.az)
		if a < a_min {
			a_min = a
			min_idx = idx
		}
	}

	return min_idx
}

func part2(content string) int {
	particles := parse(strings.Split(content, "\n"))

	n := len(particles)
	old := n
	fly := true
	for i := 1; fly; i++ {
		positions := make(map[position]*particle)
		for idx := range particles {
			p := &particles[idx]
			if p.destroyed {
				continue
			}
			if _, ok := positions[p.pos]; !ok {
				positions[p.pos] = p
			} else {
				p.destroyed = true
				n--
				if old := positions[p.pos]; !old.destroyed {
					old.destroyed = true
					n--
				}
			}
			move(p)
		}
		if i%100 == 0 {
			if n == old {
				fly = false
			}
			old = n
		}

	}
	return n
}

func move(p *particle) {
	p.vx += p.ax
	p.vy += p.ay
	p.vz += p.az
	p.pos.x += p.vx
	p.pos.y += p.vy
	p.pos.z += p.vz
}

func parse(lines []string) (particles []particle) {
	particles = make([]particle, len(lines))
	for idx, line := range lines {
		p := &particles[idx]
		fmt.Sscanf(line, "p=<%d,%d,%d>, v=<%d,%d,%d>, a=<%d,%d,%d>",
			&p.pos.x, &p.pos.y, &p.pos.z, &p.vx, &p.vy, &p.vz, &p.ax, &p.ay, &p.az)
	}
	return
}

func abs(i int64) uint {
	if i < 0 {
		return uint(-i)
	} else {
		return uint(i)
	}
}
