package main

import "testing"

func Test02Example(t *testing.T) {
	levels, err := read02File("../data/02/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevels(levels)
	if countSafe != 2 {
		t.Fatalf("Expected 2 safe leves, got=%d", countSafe)
	}
}

func Test02Input(t *testing.T) {
	levels, err := read02File("../data/02/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevels(levels)
	if countSafe != 686 {
		t.Fatalf("Expected 686 safe leves, got=%d", countSafe)
	}
}
