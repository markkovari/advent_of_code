package main

import (
	"os"
	"testing"
)

func TestReadCommands(t *testing.T) {
	content, err := os.ReadFile("../inputs/15/prod.txt")
	if err != nil {
		t.Fatalf("cannot read inputs, %v+", err)
	}
	vals := parseCommands(string(content))
	expectedLen := 1045
	if len(vals) != expectedLen {
		t.Fatalf("len of inputs should be %d, got %d", expectedLen, len(vals))
	}

	expectedExampleLen := 8
	exampleContent, err := os.ReadFile("../inputs/15/test.txt")
	if err != nil {
		t.Fatalf("cannot read inputs, %v+", err)
	}
	example := parseCommands(string(exampleContent))
	if len(example) != expectedExampleLen {
		t.Fatalf("len of example inputs should be %d, got %d", expectedExampleLen, len(example))
	}
}

func TestDay15(t *testing.T) {
	d := Day15{}
	content, err := os.ReadFile("../inputs/15/prod.txt")
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(content)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := "Not Implemented"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := "Not Implemented"
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}
