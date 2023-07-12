package days

import "testing"

func Test(t *testing.T) {
	t.Skip("Skipping slow test")
	t.Run("Day17First", func(t *testing.T) {
		println("starting day 17 first tests")
		result, err := Day17First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 378
		if result != want {
			t.Errorf("Day17First(false) = %d; want %d", result, want)
		}
		result, err = Day17First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1244
		if result != want {
			t.Errorf("Day17First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day17Second", func(t *testing.T) {
		result, err := Day17Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 42440801
		if result != want {
			t.Errorf("Day17Second(false) = %d; want %d", result, want)
		}
		result, err = Day17Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 11162912
		if result != want {
			t.Errorf("Day17Second(false) = %d; want %d", result, want)
		}
	})
}
