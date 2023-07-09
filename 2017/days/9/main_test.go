package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day9First", func(t *testing.T) {

		result, err := Day9First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 3
		if result != want {
			t.Errorf("Day9First(false) = %d; want %d", result, want)
		}
		result, err = Day9First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 23588
		if result != want {
			t.Errorf("Day9First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day9Second", func(t *testing.T) {

		result, err := Day9Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 17
		if result != want {
			t.Errorf("Day9Second(false) = %d; want %d", result, want)
		}
		result, err = Day9Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 10045
		if result != want {
			t.Errorf("Day9Second(false) = %d; want %d", result, want)
		}
	})
}
