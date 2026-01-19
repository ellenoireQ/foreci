// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package start

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

type StartResult struct {
	ContainerID string `json:"container_id"`
	Status      string `json:"status"`
	Error       string `json:"error,omitempty"`
}

var StartCmd = &cobra.Command{
	Use:   "start [container_id]",
	Short: "Start a Docker container",
	Long:  `Start a stopped Docker container using its container ID or name`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		containerID := args[0]
		startContainer(containerID)
	},
}

func outputJSON(result StartResult) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(result)
}

func startContainer(containerID string) {
	ctx := context.Background()

	cli, err := client.New(client.FromEnv)
	if err != nil {
		outputJSON(StartResult{
			ContainerID: containerID,
			Status:      "error",
			Error:       fmt.Sprintf("failed to create docker client: %v", err),
		})
		return
	}
	defer cli.Close()

	_, err = cli.ContainerStart(ctx, containerID, client.ContainerStartOptions{})
	if err != nil {
		outputJSON(StartResult{
			ContainerID: containerID,
			Status:      "error",
			Error:       fmt.Sprintf("failed to start container: %v", err),
		})
		return
	}

	inspect, err := cli.ContainerInspect(ctx, containerID, client.ContainerInspectOptions{})
	if err != nil {
		outputJSON(StartResult{
			ContainerID: containerID,
			Status:      "started",
		})
		return
	}

	outputJSON(StartResult{
		ContainerID: inspect.Container.ID,
		Status:      string(inspect.Container.State.Status),
	})
}
