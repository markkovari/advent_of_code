package day06

import (
	"fmt"
	"strings"
)

type Solver struct{}

func (d Solver) Part1(input string) string {
	banks := d.parse(input)
	seen := make(map[[16]int]int)
	cycles := 0
	for {
		if _, ok := seen[banks]; ok {
			return fmt.Sprintf("%d", cycles)
		}
		seen[banks] = cycles

		maxIdx, maxVal := 0, banks[0]
		for i, val := range banks {
			if val > maxVal {
				maxIdx, maxVal = i, val
			}
		}

		banks[maxIdx] = 0
		for i := 1; i <= maxVal; i++ {
			banks[(maxIdx+i)%16]++
		}
		cycles++
	}
}

func (d Solver) Part2(input string) string {
	banks := d.parse(input)
	seen := make(map[[16]int]int)
	cycles := 0
	for {
		if firstSeen, ok := seen[banks]; ok {
			return fmt.Sprintf("%d", cycles-firstSeen)
		}
		seen[banks] = cycles

		maxIdx, maxVal := 0, banks[0]
		for i, val := range banks {
			if val > maxVal {
				maxIdx, maxVal = i, val
			}
		}

		banks[maxIdx] = 0
		for i := 1; i <= maxVal; i++ {
			banks[(maxIdx+i)%16]++
		}
		cycles++
	}
}

func (d Solver) parse(input string) [16]int {
	var banks [16]int
	for i, s := range strings.Fields(input) {
		fmt.Sscanf(s, "%d", &banks[i])
	}
	return banks
}
