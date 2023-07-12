package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day20First", func(t *testing.T) {
		result, err := Day20First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		if result != want {
			t.Errorf("Day20First(false) = %d; want %d", result, want)
		}
		result, err = Day20First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 125
		if result != want {
			t.Errorf("Day20First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day20Second", func(t *testing.T) {
		result, err := Day20Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		if result != want {
			t.Errorf("Day20Second(false) = %d; want %d", result, want)
		}
		result, err = Day20Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 461
		if result != want {
			t.Errorf("Day20Second(false) = %d; want %d", result, want)
		}
	})
}
