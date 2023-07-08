package days

import "testing"

func TestSecond(t *testing.T) {
	t.Run("Day2First", func(t *testing.T) {

		result, err := Day2First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 18
		if result != want {
			t.Errorf("Day2First(false) = %d; want %d", result, want)
		}
		result, err = Day2First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 43074
		if result != want {
			t.Errorf("Day2First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day2Second", func(t *testing.T) {

		result, err := Day2Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 9
		if result != want {
			t.Errorf("Day2Second(false) = %d; want %d", result, want)
		}
		result, err = Day2Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 280
		if result != want {
			t.Errorf("Day2Second(false) = %d; want %d", result, want)
		}
	})
}
