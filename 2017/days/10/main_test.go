package days

import "testing"

func Test(t *testing.T) {
	t.Run("Day10First", func(t *testing.T) {

		result, err := Day10First(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := 0
		if result != want {
			t.Errorf("Day10First(false) = %d; want %d", result, want)
		}
		result, err = Day10First(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = 1935
		if result != want {
			t.Errorf("Day10First(false) = %d; want %d", result, want)
		}
	})

	t.Run("Day10Second", func(t *testing.T) {

		result, err := Day10Second(false)
		if err != nil {
			t.Error("Should not fail")
		}
		want := "212c9636ab5f11d2931b51383f430601"
		if result != want {
			t.Errorf("Day10Second(false) = %s; want %s", result, want)
		}
		result, err = Day10Second(true)
		if err != nil {
			t.Error("Should not fail")
		}
		want = "dc7e7dee710d4c7201ce42713e6b8359"
		if result != want {
			t.Errorf("Day10Second(false) = %s; want %s", result, want)
		}
	})
}
