// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package cmd

import (
	"foreci/runner/cmd/read"
	runjob "foreci/runner/cmd/run-job"
	searchmatch "foreci/runner/cmd/search-match"
	"log"

	"github.com/spf13/cobra"
)

var runCmd = &cobra.Command{
	Use:   "run [job]",
	Short: "Run a CI job",
	Long:  `Run a specific CI job and stream output as JSON.`,
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		jobName := args[0]
		runjob.RunJob(jobName)

	},
}

/*
* WIP -- Function
* Reading Docker build system file
* @param: main.go read {dockerfile, compose} $pathfile
* */
var readCmd = &cobra.Command{
	Use:   "read [type] [path]",
	Short: "Read docker files",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		fileType := args[0] // <== dockerfile | compose
		path := args[1]

		switch fileType {
		case "compose":
			read.ReadCompose(path)
		case "dockerfile":
			read.ReadDocker(path)
		default:
			log.Fatalf("unknown type: %s", fileType)
		}
	},
}

var searchMatchesCmd = &cobra.Command{
	Use:   "search $PREFIX",
	Short: "Search matches files",
	Args:  cobra.MaximumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		prefix := args[0]
		searchmatch.SearchMatchesFile(prefix)
	},
}

func init() {
	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(readCmd)
	rootCmd.AddCommand(searchMatchesCmd)
}
