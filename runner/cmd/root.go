// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package cmd

import (
	"fmt"
	"os"

	"foreci/runner/cmd/images"

	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "foreci",
	Short: "foreci CI/CD Tools",
	Long:  `A continuous integration tool designed for easy integration with docker build systems`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("Foreci Runner v0.1.0")
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
}
