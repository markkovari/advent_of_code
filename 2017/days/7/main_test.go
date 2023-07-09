package days

import "testing"

func TestFourth(t *testing.T) {
	t.Run("Day7First", func(t *testing.T) {

		result, err := Day7First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := "tknk"
		if result != want {
			t.Errorf("Day7First(false) = %s; want %s", result, want)
		}
		result, err = Day7First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = "vmpywg"
		if result != want {
			t.Errorf("Day6First(false) = %s; want %s", result, want)
		}
	})

	t.Run("Day7Second", func(t *testing.T) {

		result, err := Day7Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 60
		if result != want {
			t.Errorf("Day7Second(false) = %d; want %d", result, want)
		}
		result, err = Day7Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1674
		if result != want {
			t.Errorf("Day6Second(false) = %d; want %d", result, want)
		}
	})
}
