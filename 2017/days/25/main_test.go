package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day25First", func(t *testing.T) {
		result, err := Day25First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 3
		if result != want {
			t.Errorf("Day25First(false) = %d; want %d", result, want)
		}
		result, err = Day25First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 5593
		if result != want {
			t.Errorf("Day25First(false) = %d; want %d", result, want)
		}
	})
}
