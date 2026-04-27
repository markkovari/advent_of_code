# 🐹 Golang Advent of Code

This directory contains Go solutions for Advent of Code, structured as independent modules sharing a common utility library.

## 📁 Structure

Each year is a separate module that uses a local `replace` directive to point to the shared `common` library.

- **`common/`**: Shared interface and input resolution logic.
- **`20XX/`**: Year-specific implementations and dispatchers.

## 🚀 Standardized Day Template

Every day is implemented in a separate file (e.g., `day01.go`) within the year directory:

```go
package main

import (
	"fmt"
	"github.com/markkovari/advent_of_code/golang/common"
)

type DayXX struct{}

func (d DayXX) Part1(input string) string {
	return "result"
}

func (d DayXX) Part2(input string) string {
	return "result"
}
```

## 🧪 Running Solutions

Go solutions are run through a centralized `main.go` in each year directory.

```bash
# Navigate to the year
cd golang/2024

# Run a specific day
go run . 1

# Run tests
go test ./...
```

## 🛠 Adding a New Year

1. Create a folder: `mkdir -p 20XX`.
2. Initialize module: `go mod init github.com/markkovari/advent_of_code/golang/20XX`.
3. Add `replace` to `go.mod`: `replace github.com/markkovari/advent_of_code/golang/common => ../common`.
4. Create your `dayXX.go` files and a `main.go` dispatcher.
