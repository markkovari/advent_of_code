package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Day10 struct{}

func (d Day10) Part1(input string) string {
	list := make([]int, 256)
	for i := range list {
		list[i] = i
	}
	lengths := d.parse(input)
	pos, skip := 0, 0
	for _, l := range lengths {
		d.reverse(&list, pos, l)
		pos = (pos + l + skip) % 256
		skip++
	}
	return fmt.Sprintf("%d", list[0]*list[1])
}

func (d Day10) Part2(input string) string {
	list := make([]int, 256)
	for i := range list {
		list[i] = i
	}
	var lengths []int
	for _, b := range []byte(strings.TrimSpace(input)) {
		lengths = append(lengths, int(b))
	}
	lengths = append(lengths, []int{17, 31, 73, 47, 23}...)
	pos, skip := 0, 0
	for round := 0; round < 64; round++ {
		for _, l := range lengths {
			d.reverse(&list, pos, l)
			pos = (pos + l + skip) % 256
			skip++
		}
	}
	var hash string
	for i := 0; i < 16; i++ {
		block := 0
		for j := 0; j < 16; j++ {
			block ^= list[i*16+j]
		}
		hash += fmt.Sprintf("%02x", block)
	}
	return hash
}

func (d Day10) reverse(list *[]int, pos, l int) {
	for i := 0; i < l/2; i++ {
		a, b := (pos+i)%256, (pos+l-1-i)%256
		(*list)[a], (*list)[b] = (*list)[b], (*list)[a]
	}
}

func (d Day10) parse(input string) []int {
	var l []int
	for _, s := range strings.Split(strings.TrimSpace(input), ",") {
		v, _ := strconv.Atoi(s)
		l = append(l, v)
	}
	return l
}
