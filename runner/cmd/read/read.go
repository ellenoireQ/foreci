// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package read

import (
	"encoding/json"
	"os"
	"path/filepath"

	"github.com/moby/buildkit/frontend/dockerfile/parser"

	"github.com/compose-spec/compose-go/loader"
	"github.com/compose-spec/compose-go/types"
)

type DockerFile struct {
	Value    string `json:"value"`
	Original string `json:"original"`
}

type DockerCompose struct {
	Name          string   `json:"name"`
	Service       string   `json:"service"`
	Image         string   `json:"image"`
	Ports         string   `json:"ports"`
	ContainerName string   `json:"container_name"`
	Hostname      string   `json:"hostname"`
	BuildContext  string   `json:"build_context"`
	Dockerfile    string   `json:"dockerfile"`
	Environment   []string `json:"environment"`
	Volumes       []string `json:"volumes"`
	Networks      []string `json:"networks"`
	Restart       string   `json:"restart"`
}

func outputDockerJSON(o DockerFile) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(o)
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
		outputDockerJSON(DockerFile{
			Value:    child.Value,
			Original: child.Original,
		})
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
		var envVars []string
		for key, val := range svc.Environment {
			if val != nil {
				envVars = append(envVars, key+"="+*val)
			} else {
				envVars = append(envVars, key)
			}
		}

		var volumes []string
		for _, vol := range svc.Volumes {
			volumes = append(volumes, vol.String())
		}

		var networks []string
		for netName := range svc.Networks {
			networks = append(networks, netName)
		}

		var buildContext, dockerfile string
		if svc.Build != nil {
			buildContext = svc.Build.Context
			dockerfile = svc.Build.Dockerfile
		}

		var ports []string
		for _, port := range svc.Ports {
			if port.Published != "" {
				ports = append(ports, port.Published)
			}
		}

		var portsStr string
		if len(ports) > 0 {
			portsStr = ports[0]
			for i := 1; i < len(ports); i++ {
				portsStr += ", " + ports[i]
			}
		}

		outputJSON(DockerCompose{
			Name:          project.Name,
			Service:       svc.Name,
			Image:         svc.Image,
			Ports:         portsStr,
			ContainerName: svc.ContainerName,
			Hostname:      svc.Hostname,
			BuildContext:  buildContext,
			Dockerfile:    dockerfile,
			Environment:   envVars,
			Volumes:       volumes,
			Networks:      networks,
			Restart:       svc.Restart,
		})
	}
}
