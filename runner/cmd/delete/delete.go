// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package delete

import (
	"context"
	"fmt"
	"log"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

var DeleteContainerCmd = &cobra.Command{
	Use:   "rm [container_id]",
	Short: "Delete container",
	Long:  `Delete a Docker container by container ID`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		containerID := args[0]
		DeleteContainer(containerID)
	},
}

func DeleteContainer(containerID string) {
	ctx := context.Background()
	cli, err := client.New(client.FromEnv)
	if err != nil {
		fmt.Printf("[Error] Error creating Docker client: %v\n", err)
		return
	}
	defer cli.Close()

	options := client.ContainerRemoveOptions{
		Force:         true,
		RemoveVolumes: false,
	}

	_, err = cli.ContainerRemove(ctx, containerID, options)
	if err != nil {
		fmt.Printf("[Error] Error removing container %s: %v\n", containerID, err)
		return
	}

	fmt.Printf("[Success] Container %s has been removed\n", containerID)
}

var DeleteImageCmd = &cobra.Command{
	Use:   "rmi [image_id]",
	Short: "Delete image",
	Long:  `Delete image based by image id`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		imageID := args[0]
		DeleteExistingImage(imageID)
	},
}

func DeleteExistingImage(imageID string) {
	ctx := context.Background()
	cli, err := client.New(client.FromEnv)
	if err != nil {
		log.Fatalf("Error creating Docker client: %v", err)
	}

	options := client.ImageRemoveOptions{
		// TODO:: Changing to false after implementing "Force Delete" menu
		Force:         true,
		PruneChildren: true,
	}

	dels, err := cli.ImageRemove(ctx, imageID, options)
	if err != nil {
		log.Fatalf("Error removing image %s: %v", imageID, err)
	}

	fmt.Printf("Image(s) removed successfully:\n")
	for _, del := range dels.Items {
		if del.Deleted != "" {
			fmt.Printf("Deleted: %s\n", del.Deleted)
		}
		if del.Untagged != "" {
			fmt.Printf("Untagged: %s\n", del.Untagged)
		}
	}
}
