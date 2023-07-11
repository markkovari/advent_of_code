package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day15First", func(t *testing.T) {
		result, err := Day15First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 588
		if result != want {
			t.Errorf("Day15First(false) = %d; want %d", result, want)
		}
		result, err = Day15First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 619
		if result != want {
			t.Errorf("Day15First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day15Second", func(t *testing.T) {
		result, err := Day15Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 309
		if result != want {
			t.Errorf("Day15Second(false) = %d; want %d", result, want)
		}
		result, err = Day15Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 290
		if result != want {
			t.Errorf("Day15Second(false) = %d; want %d", result, want)
		}
	})
}
