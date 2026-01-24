// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package stop

import (
	"context"
	"encoding/json"
	"os"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

type StopResult struct {
	ContainerID string `json:"container_id"`
	Status      string `json:"status"`
	Error       string `json:"error,omitempty"`
}

var StopCmd = &cobra.Command{
	Use:   "stop [container_id]",
	Short: "Stopping a Docker container",
	Long:  `Stopping a stopped Docker container using container ID`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		containerID := args[0]
		StopContainer(containerID)
	},
}

func outputJSON(result StopResult) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(result)
}

func StopContainer(containerID string) {
	ctx := context.Background()
	cli, err := client.New(client.FromEnv)
	if err != nil {
		outputJSON(StopResult{
			ContainerID: containerID,
			Status:      "error",
			Error:       err.Error(),
		})
		return
	}
	defer cli.Close()

	_, err = cli.ContainerStop(ctx, containerID, client.ContainerStopOptions{})
	if err != nil {
		outputJSON(StopResult{
			ContainerID: containerID,
			Status:      "error",
			Error:       err.Error(),
		})
		return
	}

	outputJSON(StopResult{
		ContainerID: containerID,
		Status:      "stopped",
	})
}
