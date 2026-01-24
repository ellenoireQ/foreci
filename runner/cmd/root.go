// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package cmd

import (
	"fmt"
	"os"

	"easydocker/runner/cmd/create"
	"easydocker/runner/cmd/delete"
	"easydocker/runner/cmd/images"
	"easydocker/runner/cmd/list"
	"easydocker/runner/cmd/start"
	"easydocker/runner/cmd/stop"
	"easydocker/runner/cmd/stream"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "easydocker",
	Short: "easydocker - Docker Management Tool",
	Long:  `A continuous integration tool designed for easy integration with docker build systems`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Easydocker Runner v0.1.0")
		fmt.Println("Use --help for available commands")
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(images.ImagesCmd)
	rootCmd.AddCommand(start.StartCmd)
	rootCmd.AddCommand(create.CreateCmd)
	rootCmd.AddCommand(list.ListCmd)
	rootCmd.AddCommand(stream.StreamCmd)
	rootCmd.AddCommand(stop.StopCmd)
	rootCmd.AddCommand(delete.DeleteImageCmd)
}
