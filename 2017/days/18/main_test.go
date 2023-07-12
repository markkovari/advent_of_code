package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day18First", func(t *testing.T) {
		println("starting day 17 first tests")
		result, err := Day18First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 4
		if result != want {
			t.Errorf("Day18First(false) = %d; want %d", result, want)
		}
		result, err = Day18First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 9423
		if result != want {
			t.Errorf("Day18First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day18Second", func(t *testing.T) {
		result, err := Day18Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		if result != want {
			t.Errorf("Day18Second(false) = %d; want %d", result, want)
		}
		result, err = Day18Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 7620
		if result != want {
			t.Errorf("Day18Second(false) = %d; want %d", result, want)
		}
	})
}
