package days

import "testing"

func TestFirst(t *testing.T) {
	if First() != 1 {
		t.Error("First() should return 2")
	}

}
