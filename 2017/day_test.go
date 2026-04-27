package main

import (
	"os"
	"testing"
	"fmt"
)

func TestDay01(t *testing.T) {
	d := Day01{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 1))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 1044)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1054)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay02(t *testing.T) {
	d := Day02{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 2))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 43074)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 280)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay03(t *testing.T) {
	d := Day03{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 3))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 480)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 349975)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay04(t *testing.T) {
	d := Day04{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 4))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 325)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 119)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay05(t *testing.T) {
	d := Day05{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 5))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 351282)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 24568703)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay06(t *testing.T) {
	d := Day06{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 6))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 7864)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1695)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay07(t *testing.T) {
	d := Day07{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 7))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", "vmpywg")
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1674)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay08(t *testing.T) {
	d := Day08{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 8))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 4902)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 7037)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay09(t *testing.T) {
	d := Day09{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 9))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 23588)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 10045)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay10(t *testing.T) {
	d := Day10{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 10))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 1935)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", "dc7e7dee710d4c7201ce42713e6b8359")
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay11(t *testing.T) {
	d := Day11{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 11))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 643)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1471)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay12(t *testing.T) {
	d := Day12{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 12))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 113)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 202)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay13(t *testing.T) {
	d := Day13{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 13))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 1640)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 3960702)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay14(t *testing.T) {
	d := Day14{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 14))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 8074)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1212)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay15(t *testing.T) {
	d := Day15{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 15))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 619)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 290)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay16(t *testing.T) {
	d := Day16{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 16))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", "cknmidebghlajpfo")
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", "cbolhmkgfpenidaj")
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay17(t *testing.T) {
	d := Day17{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 17))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 1244)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 11162912)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay18(t *testing.T) {
	d := Day18{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 18))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 9423)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 7620)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay19(t *testing.T) {
	d := Day19{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 19))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", "UICRNSDOK")
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 16064)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay20(t *testing.T) {
	d := Day20{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 20))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 125)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 461)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay21(t *testing.T) {
	d := Day21{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 21))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 133)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 2221990)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay22(t *testing.T) {
	d := Day22{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 22))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 5322)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 2512079)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay23(t *testing.T) {
	d := Day23{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 23))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 6241)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 909)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay24(t *testing.T) {
	d := Day24{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 24))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 1511)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
	t.Run("Part2", func(t *testing.T) {
		got := d.Part2(inputStr)
		want := fmt.Sprintf("%v", 1471)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

func TestDay25(t *testing.T) {
	d := Day25{}
	input, err := os.ReadFile(fmt.Sprintf("inputs/%d/prod.txt", 25))
	if err != nil {
		t.Fatalf("Could not read input: %v", err)
	}
	inputStr := string(input)

	t.Run("Part1", func(t *testing.T) {
		got := d.Part1(inputStr)
		want := fmt.Sprintf("%v", 5593)
		if got != want {
			t.Errorf("got %q, want %q", got, want)
		}
	})
}

