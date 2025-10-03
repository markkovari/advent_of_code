package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strings"
)

type InputType int

const (
	example InputType = iota
	input
)

const (
	inputPath   string = "../../data/input"
	examplePath string = "../../data/example"
)

var inputs map[InputType]string = map[InputType]string{
	example: examplePath,
	input:   inputPath,
}

func get_04_data(version InputType) ([][]string, error) {
	path, ok := inputs[version]
	if !ok {
		return [][]string{}, fmt.Errorf("Cannot find path")
	}
	file, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("failed to open file: %w", err)
	}
	defer file.Close()

	var data [][]string

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		fields := strings.Fields(line)

		data = append(data, fields)
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("error reading file: %w", err)
	}

	return data, nil
}
