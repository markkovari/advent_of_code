package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day22First", func(t *testing.T) {
		result, err := Day22First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 5587
		if result != want {
			t.Errorf("Day22First(false) = %d; want %d", result, want)
		}
		result, err = Day22First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 5322
		if result != want {
			t.Errorf("Day22First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day22Second", func(t *testing.T) {
		result, err := Day22Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 2511944
		if result != want {
			t.Errorf("Day22Second(false) = %d; want %d", result, want)
		}
		result, err = Day22Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 2512079
		if result != want {
			t.Errorf("Day22Second(false) = %d; want %d", result, want)
		}
	})
}
