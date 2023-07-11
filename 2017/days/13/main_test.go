package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day13First", func(t *testing.T) {
		result, err := Day13First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 24
		if result != want {
			t.Errorf("Day13First(false) = %d; want %d", result, want)
		}
		result, err = Day13First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1640
		if result != want {
			t.Errorf("Day13First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day13Second", func(t *testing.T) {
		t.Skip("eh")
		result, err := Day13Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 10
		if result != want {
			t.Errorf("Day13Second(false) = %d; want %d", result, want)
		}
		result, err = Day13Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 3960702
		if result != want {
			t.Errorf("Day13Second(false) = %d; want %d", result, want)
		}
	})
}
