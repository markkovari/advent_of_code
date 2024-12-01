package main

import "testing"

func TestExample(t *testing.T) {
	left, right, err := readFile("../data/01/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	left, right = sortInput(left, right)
	distances := getDistances(left, right)
	distanceSum := sumDistances(distances)
	if distanceSum != 11 {
		t.Fatalf("Expected 11, got %d", distanceSum)
	}
}

func TestInput(t *testing.T) {
	left, right, err := readFile("../data/01/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	left, right = sortInput(left, right)
	distances := getDistances(left, right)
	distanceSum := sumDistances(distances)
	if distanceSum != 1970720 {
		t.Fatalf("Expected 1970720, got %d", distanceSum)
	}
}
