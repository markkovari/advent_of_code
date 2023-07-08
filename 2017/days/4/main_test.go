package days

import "testing"

func TestFourth(t *testing.T) {
	t.Run("Day4First", func(t *testing.T) {

		result, err := Day4First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 2
		if result != want {
			t.Errorf("Day4First(false) = %d; want %d", result, want)
		}
		result, err = Day4First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 325
		if result != want {
			t.Errorf("Day4First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day4Second", func(t *testing.T) {

		result, err := Day4Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 3
		if result != want {
			t.Errorf("Day4Second(false) = %d; want %d", result, want)
		}
		result, err = Day4Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 119
		if result != want {
			t.Errorf("Day4Second(false) = %d; want %d", result, want)
		}
	})
}
