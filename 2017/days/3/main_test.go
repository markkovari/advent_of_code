package days

import "testing"

func TestDistance(t *testing.T) {
	t.Run("Distance", func(t *testing.T) {
		result := part1("1")
		want := 0
		if result != want {
			t.Errorf("part1(1) = %d; want %d", result, want)
		}
		result = part1("12")
		want = 3
		if result != want {
			t.Errorf("part1(12) = %d; want %d", result, want)
		}
		result = part1("23")
		want = 2
		if result != want {
			t.Errorf("part1(23) = %d; want %d", result, want)
		}
		result = part1("1024")
		want = 31
		if result != want {
			t.Errorf("part1(1024) = %d; want %d", result, want)
		}
	})

}

func TestThird(t *testing.T) {
	t.Run("Day3First", func(t *testing.T) {

		result, err := Day3First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 31
		if result != want {
			t.Errorf("Day3First(false) = %d; want %d", result, want)
		}
		result, err = Day3First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 480
		if result != want {
			t.Errorf("Day3First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day3Second", func(t *testing.T) {

		result, err := Day3Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1968
		if result != want {
			t.Errorf("Day3Second(false) = %d; want %d", result, want)
		}
		result, err = Day3Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 349975
		if result != want {
			t.Errorf("Day3Second(false) = %d; want %d", result, want)
		}
	})
}
