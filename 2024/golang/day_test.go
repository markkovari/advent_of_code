package main

import (
	"fmt"
	"os"
	"testing"
)

func findTestInput(day int, name string) string {
	return fmt.Sprintf("../inputs/%d/%s.txt", day, name)
}

func TestDay01(t *testing.T) {
	d := Day01{}
	input, _ := os.ReadFile(findTestInput(1, "prod"))
	inputStr := string(input)
	
	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := "1970720"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := "17191599"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay02(t *testing.T) {
	d := Day02{}
	input, _ := os.ReadFile(findTestInput(2, "prod"))
	inputStr := string(input)
	
	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := "686"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := "717"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay03(t *testing.T) {
	d := Day03{}
	input, _ := os.ReadFile(findTestInput(3, "prod"))
	inputStr := string(input)
	
	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := "173785482"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := "83158140"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}
