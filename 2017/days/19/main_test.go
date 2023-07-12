package days

import "testing"

func Test(t *testing.T) {
	t.Skip("Skipping day 19 tests")
	t.Run("Day19First", func(t *testing.T) {
		// result, err := Day19First(false)
		// if err != nil {
		// 	t.Error("Should not fail")
		// }
		// want := "ABCDEF"
		// if result != want {
		// 	t.Errorf("Day19First(false) = %s; want %s", result, want)
		// }
		result, err := Day19First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want := "UICRNSDOK"
		if result != want {
			t.Errorf("Day19First(false) = %s; want %s", result, want)
		}
	})

	t.Run("Day19Second", func(t *testing.T) {
		result, err := Day19Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 16064
		if result != want {
			t.Errorf("Day19Second(false) = %d; want %d", result, want)
		}
		result, err = Day19Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 16064
		if result != want {
			t.Errorf("Day19Second(false) = %d; want %d", result, want)
		}
	})
}
