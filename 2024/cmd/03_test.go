package main

import (
	"testing"
)

func Test03Example(t *testing.T) {
	content, err := read03File("../data/03/example")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	muls := getMuls(content)

	sum := getSum(muls)
	if sum != 161 {
		t.Fatalf("Unexpected sum 161, but got=%d", sum)
	}
}

func Test03Input(t *testing.T) {
	content, err := read03File("../data/03/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	muls := getMuls(content)

	sum := getSum(muls)
	if sum != 173785482 {
		t.Fatalf("Unexpected sum 173785482, but got=%d", sum)
	}
}

func Test03ExampleSecond(t *testing.T) {
	content, err := read03File("../data/03/example_2")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	content = stripDontParts(content)
	muls := getMuls(content)
	sum := getSum(muls)
	if sum != 48 {
		t.Fatalf("Unexpected sum 48, but got=%d", sum)
	}
}

func Test03InputSecond(t *testing.T) {
	content, err := read03File("../data/03/input")
	if err != nil {
		t.Fatalf("Error reading file: %v", err)
	}
	values := findEnabledMultOps(content)

	sum := sumValues(values)

	if sum != 83158140 {
		t.Fatalf("Unexpected sum 83158140, but got=%d", sum)
	}
}
