package main

import "testing"

func test_simple(t *testing.T) {
	if 1 != 1 {
		t.Fatal("yikes")
	}
}
