package read

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/moby/buildkit/frontend/dockerfile/parser"

	"github.com/compose-spec/compose-go/loader"
	"github.com/compose-spec/compose-go/types"
)

type DockerCompose struct {
	Name    string `json:"name"`
	Service string `json:"service"`
	Image   string `json:"image"`
	Ports   string `json:"ports"`
}

func outputJSON(o DockerCompose) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(o)
}

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

func ReadCompose(path string) {
	abs, err := filepath.Abs(path)
	if err != nil {
		panic(err)
	}

	workdir := filepath.Dir(abs)

	config := types.ConfigDetails{
		WorkingDir: workdir,
		ConfigFiles: []types.ConfigFile{
			{
				Filename: abs,
			},
		},
		Environment: map[string]string{},
	}

	project, err := loader.Load(config)
	if err != nil {
		panic(err)
	}
	project.Name = loader.NormalizeProjectName(
		filepath.Base(filepath.Dir(path)),
	)

	for _, svc := range project.Services {
		// Reading and format into json
		for _, port := range svc.Ports {
			outputJSON(DockerCompose{
				Name:    project.Name,
				Service: svc.Name,
				Image:   svc.Image,
				Ports:   port.Published,
			})
		}
	}
}
