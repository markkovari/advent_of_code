package days

import "testing"

func TestFirst(t *testing.T) {
	t.Run("Day1First", func(t *testing.T) {

		result, err := Day1First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 9
		if result != want {
			t.Errorf("Day1First(false) = %d; want %d", result, want)
		}
		result, err = Day1First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1043
		if result != want {
			t.Errorf("Day1First(false) = %d; want %d", result, want)
		}
	})
}
