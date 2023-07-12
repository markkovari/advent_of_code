package days

import "testing"

func Test(t *testing.T) {
	t.Skip("Skipping slow test")

	t.Run("Day16First", func(t *testing.T) {
		result, err := Day16First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want := "cknmidebghlajpfo"
		if result != want {
			t.Errorf("Day16First(false) = %s; want %s", result, want)
		}
	})

	t.Run("Day16Second", func(t *testing.T) {
		result, err := Day16Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want := "cbolhmkgfpenidaj"
		if result != want {
			t.Errorf("Day16Second(false) = %s; want %s", result, want)
		}
	})
}
