package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day14First", func(t *testing.T) {
		result, err := Day14First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 8108
		if result != want {
			t.Errorf("Day14First(false) = %d; want %d", result, want)
		}
		result, err = Day14First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 8074
		if result != want {
			t.Errorf("Day14First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day14Second", func(t *testing.T) {
		result, err := Day14Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 1242
		if result != want {
			t.Errorf("Day14Second(false) = %d; want %d", result, want)
		}
		result, err = Day14Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1212
		if result != want {
			t.Errorf("Day14Second(false) = %d; want %d", result, want)
		}
	})
}
