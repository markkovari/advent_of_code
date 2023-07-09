package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day11First", func(t *testing.T) {
		result, err := Day11First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 3
		if result != want {
			t.Errorf("Day11First(false) = %d; want %d", result, want)
		}
		result, err = Day11First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 643
		if result != want {
			t.Errorf("Day11First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day11Second", func(t *testing.T) {

		result, err := Day11Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 3
		if result != want {
			t.Errorf("Day11Second(false) = %d; want %d", result, want)
		}
		result, err = Day11Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1471
		if result != want {
			t.Errorf("Day11Second(false) = %d; want %d", result, want)
		}
	})
}
