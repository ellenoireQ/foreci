package cmd

import (
	"foreci/runner/cmd/read"
	"foreci/runner/cmd/run-job"
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
* Reading Docker file
* @param: main.go read $dockerfile
* */
var readDockerFile = &cobra.Command{
	Use:   "read [file]",
	Short: "Reading Dockerfile",
	Long:  `Reading Dockerfile`,
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		filePath := args[0]
		read.ReadDocker(filePath)
	},
}

func init() {
	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(readDockerFile)
}
