package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day21First", func(t *testing.T) {
		result, err := Day21First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 0
		if result != want {
			t.Errorf("Day21First(false) = %d; want %d", result, want)
		}
		result, err = Day21First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 133
		if result != want {
			t.Errorf("Day21First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day21Second", func(t *testing.T) {
		result, err := Day21Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 0
		if result != want {
			t.Errorf("Day21Second(false) = %d; want %d", result, want)
		}
		result, err = Day21Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 2221990
		if result != want {
			t.Errorf("Day21Second(false) = %d; want %d", result, want)
		}
	})
}
