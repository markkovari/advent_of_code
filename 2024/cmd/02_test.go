package main

import "testing"

func Test02Example(t *testing.T) {
	levels, err := read02File("../data/02/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevels(levels)
	if countSafe != 2 {
		t.Fatalf("Expected 2 safe levels, got=%d", countSafe)
	}
}

func Test02Input(t *testing.T) {
	levels, err := read02File("../data/02/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevels(levels)
	if countSafe != 686 {
		t.Fatalf("Expected 686 safe levels, got=%d", countSafe)
	}
}

func Test02Part2Example(t *testing.T) {
	levels, err := read02File("../data/02/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevelsTolerate(levels)
	if countSafe != 4 {
		t.Fatalf("Expected 4 safe levels, got=%d", countSafe)
	}
}

func Test02Part2Input(t *testing.T) {
	levels, err := read02File("../data/02/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	countSafe := countSafeLevelsTolerate(levels)
	if countSafe != 717 {
		t.Fatalf("Expected 717 safe levels, got=%d", countSafe)
	}
}
