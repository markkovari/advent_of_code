package main

import (
	"fmt"
	"strconv"
	"strings"
	"github.com/markkovari/advent_of_code/aoc-go-common"
)

type Day08 struct{}

func (d Day08) Part1(input string) string {
    return fmt.Sprintf("%d", solve(input, false))
}

func (d Day08) Part2(input string) string {
    return fmt.Sprintf("%d", solve(input, true))
}

func solve(input string, part2 bool) int {
    registers := make(map[string]int)
    maxSeen := 0
    for _, line := range strings.Split(input, "\n") {
        if line == "" { continue }
        p := strings.Fields(line)
        reg, op, val, condReg, condOp, condVal := p[0], p[1], p[2], p[4], p[5], p[6]
        v, _ := strconv.Atoi(val)
        cv, _ := strconv.Atoi(condVal)
        
        cond := false
        rVal := registers[condReg]
        switch condOp {
        case ">": cond = rVal > cv
        case "<": cond = rVal < cv
        case ">=": cond = rVal >= cv
        case "<=": cond = rVal <= cv
        case "==": cond = rVal == cv
        case "!=": cond = rVal != cv
        }
        
        if cond {
            if op == "inc" { registers[reg] += v } else { registers[reg] -= v }
            for _, val := range registers {
                if val > maxSeen { maxSeen = val }
            }
        }
    }
    if part2 { return maxSeen }
    maxVal := 0
    for _, val := range registers { if val > maxVal { maxVal = val } }
    return maxVal
}

func main() { common.Run(2017, 8, Day08{}) }
