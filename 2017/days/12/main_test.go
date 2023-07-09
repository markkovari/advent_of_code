package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day12First", func(t *testing.T) {
		result, err := Day12First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 6
		if result != want {
			t.Errorf("Day12First(false) = %d; want %d", result, want)
		}
		result, err = Day12First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 113
		if result != want {
			t.Errorf("Day12First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day12Second", func(t *testing.T) {

		result, err := Day12Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 2
		if result != want {
			t.Errorf("Day12Second(false) = %d; want %d", result, want)
		}
		result, err = Day12Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 202
		if result != want {
			t.Errorf("Day12Second(false) = %d; want %d", result, want)
		}
	})
}
