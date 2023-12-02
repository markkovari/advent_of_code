package internal

type AocDay struct {
	Solve func(index int, isData bool) (int, int)
}
