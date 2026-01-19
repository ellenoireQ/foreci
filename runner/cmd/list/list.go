// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package list

import (
	"context"
	"encoding/json"
	"os"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

type DockerContainer struct {
	ID      string   `json:"id"`
	Image   string   `json:"image"`
	Command string   `json:"command"`
	Created int64    `json:"created"`
	Status  string   `json:"status"`
	State   string   `json:"state"`
	Ports   []Port   `json:"ports"`
	Names   []string `json:"names"`
}

type Port struct {
	IP          string `json:"ip"`
	PrivatePort uint16 `json:"private_port"`
	PublicPort  uint16 `json:"public_port"`
	Type        string `json:"type"`
}

var ListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Docker containers",
	Long:  `List all Docker containers from the local Docker daemon`,
	Run: func(cmd *cobra.Command, args []string) {
		listContainers()
	},
}

func outputJSON(container DockerContainer) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(container)
}

func listContainers() {
	ctx := context.Background()

	cli, err := client.NewClientWithOpts(client.FromEnv, client.WithAPIVersionNegotiation())
	if err != nil {
		return
	}
	defer cli.Close()

	result, err := cli.ContainerList(ctx, client.ContainerListOptions{All: true})
	if err != nil {
		return
	}

	for _, c := range result.Items {
		var ports []Port
		for _, p := range c.Ports {
			ports = append(ports, Port{
				IP:          p.IP.String(),
				PrivatePort: p.PrivatePort,
				PublicPort:  p.PublicPort,
				Type:        p.Type,
			})
		}

		outputJSON(DockerContainer{
			ID:      c.ID[:12],
			Image:   c.Image,
			Command: c.Command,
			Created: c.Created,
			Status:  c.Status,
			State:   string(c.State),
			Ports:   ports,
			Names:   c.Names,
		})
	}
}
