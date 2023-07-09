package days

import "testing"

func TestFourth(t *testing.T) {
	t.Run("Day5First", func(t *testing.T) {

		result, err := Day5First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 5
		if result != want {
			t.Errorf("Day5First(false) = %d; want %d", result, want)
		}
		result, err = Day5First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 351282
		if result != want {
			t.Errorf("Day5First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day5Second", func(t *testing.T) {

		result, err := Day5Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 10
		if result != want {
			t.Errorf("Day5Second(false) = %d; want %d", result, want)
		}
		result, err = Day5Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 24568703
		if result != want {
			t.Errorf("Day5Second(false) = %d; want %d", result, want)
		}
	})
}
