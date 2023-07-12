package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day24First", func(t *testing.T) {
		result, err := Day24First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 31
		if result != want {
			t.Errorf("Day24First(false) = %d; want %d", result, want)
		}
		result, err = Day24First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1511
		if result != want {
			t.Errorf("Day24First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day24Second", func(t *testing.T) {
		result, err := Day24Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 19
		if result != want {
			t.Errorf("Day24Second(false) = %d; want %d", result, want)
		}
		result, err = Day24Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1471
		if result != want {
			t.Errorf("Day24Second(false) = %d; want %d", result, want)
		}
	})
}
