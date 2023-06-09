package utils

import (
	"io/ioutil"
	"os"
	"path"
)

func ReadTextsOfDay(ofDay string) (string, string, error) {
	currentDir, err := os.Getwd()
	if err != nil {
		return "", "", err
	}

	// TODO: This is a bit hacky, but it works for now. -> Probably remains like this for eternity.
	examplePath := path.Join(currentDir, "..", "..", "inputs", ofDay, "example.data")
	prodPath := path.Join(currentDir, "..", "..", "inputs", ofDay, "prod.data")
	example, err := ioutil.ReadFile(examplePath)
	if err != nil {
		return "", "", err
	}
	prod, err := ioutil.ReadFile(prodPath)
	if err != nil {
		return "", "", err
	}
	return string(example), string(prod), nil
}
