// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package stop

import (
	"context"
	"fmt"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

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

func StopContainer(containerID string) {
	ctx := context.Background()
	cli, err := client.New(client.FromEnv)
	if err != nil {
		fmt.Printf("Container with ID %s has an error %s", containerID, err.Error())
		return
	}
	defer cli.Close()

	_, err = cli.ContainerStop(ctx, containerID, client.ContainerStopOptions{})
	if err != nil {
		fmt.Printf("Container with ID %s has an error %s", containerID, err.Error())
		return
	}

	fmt.Printf("Container with ID %s has been stopped", containerID)
}
