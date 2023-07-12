package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day23First", func(t *testing.T) {
		// result, err := Day23First(false)
		// if err != nil {
		// 	t.Error("Should not fail")
		// }
		// want := 5587
		// if result != want {
		// 	t.Errorf("Day23First(false) = %d; want %d", result, want)
		// }
		result, err := Day23First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 6241
		if result != want {
			t.Errorf("Day23First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day23Second", func(t *testing.T) {
		result, err := Day23Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 909
		if result != want {
			t.Errorf("Day23Second(false) = %d; want %d", result, want)
		}
		result, err = Day23Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 909
		if result != want {
			t.Errorf("Day23Second(false) = %d; want %d", result, want)
		}
	})
}
