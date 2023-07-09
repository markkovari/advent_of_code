package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day8First", func(t *testing.T) {

		result, err := Day8First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		if result != want {
			t.Errorf("Day8First(false) = %d; want %d", result, want)
		}
		result, err = Day8First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 4902
		if result != want {
			t.Errorf("Day8First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day8Second", func(t *testing.T) {

		result, err := Day8Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 10
		if result != want {
			t.Errorf("Day8Second(false) = %d; want %d", result, want)
		}
		result, err = Day8Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 7037
		if result != want {
			t.Errorf("Day8Second(false) = %d; want %d", result, want)
		}
	})
}
