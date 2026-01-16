package read

import (
	"fmt"
	"os"

	"github.com/moby/buildkit/frontend/dockerfile/parser"
)

func ReadDocker(path string) {
	f, _ := os.Open(path)
	res, err := parser.Parse(f)
	if err != nil {
		panic(err)
	}

	for _, child := range res.AST.Children {
		fmt.Printf("%s | %s\n", child.Value, child.Original)
	}
}
