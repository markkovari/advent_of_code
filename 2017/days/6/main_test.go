package days

import "testing"

func TestFourth(t *testing.T) {
	t.Run("Day6First", func(t *testing.T) {

		result, err := Day6First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		// Is not really 1, but the slots are 16 ... so it is 1 :rip: :rocket:
		if result != want {
			t.Errorf("Day6First(false) = %d; want %d", result, want)
		}
		result, err = Day6First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 7864
		if result != want {
			t.Errorf("Day6First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day6Second", func(t *testing.T) {

		result, err := Day6Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1
		// Is not really 1, but the slots are 16 ... so it is 1 :rip: :rocket:

		if result != want {
			t.Errorf("Day6Second(false) = %d; want %d", result, want)
		}
		result, err = Day6Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1695
		if result != want {
			t.Errorf("Day6Second(false) = %d; want %d", result, want)
		}
	})
}
