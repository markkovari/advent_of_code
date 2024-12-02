package main

import "testing"

func Test01Example(t *testing.T) {
	left, right, err := read01File("../data/01/example")
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

func Test01ExampleSecond(t *testing.T) {
	left, right, err := read01File("../data/01/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	distances := getDistancesWithSimilarity(left, right)
	distanceSum := sumDistances(distances)
	if distanceSum != 31 {
		t.Fatalf("Expected 31, got %d", distanceSum)
	}
}

func Test01Input(t *testing.T) {
	left, right, err := read01File("../data/01/input")
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

func Test01InputSecond(t *testing.T) {
	left, right, err := read01File("../data/01/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	distances := getDistancesWithSimilarity(left, right)
	distanceSum := sumDistances(distances)
	if distanceSum != 17191599 {
		t.Fatalf("Expected 17191599, got %d", distanceSum)
	}
}
