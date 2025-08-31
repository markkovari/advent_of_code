package main

import (
	"testing"
)

func TestReadCommands(t *testing.T) {
	vals, err := readCommands("input.data")
	if err != nil {
		t.Fatalf("cannot read inputs, %v+", err)
	}
	expectedLen := 1044
	if len(vals) != expectedLen {
		t.Fatalf("len of inputs should be %d, got %d", expectedLen, len(vals))
	}

	expectedExampleLen := 7
	example, err := readCommands("./input.example")
	if err != nil {
		t.Fatalf("cannot read inputs, %v+", err)
	}
	if len(example) != expectedExampleLen {
		t.Fatalf("len of example inputs should be %d, got %d", expectedExampleLen, len(example))
	}
}
