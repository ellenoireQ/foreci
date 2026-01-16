package cmd

import (
	"encoding/json"
	"os"
	"time"

	"foreci/runner/cmd/read"
	"github.com/spf13/cobra"
)

var runCmd = &cobra.Command{
	Use:   "run [job]",
	Short: "Run a CI job",
	Long:  `Run a specific CI job and stream output as JSON.`,
	Args:  cobra.MinimumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		jobName := args[0]
		runJob(jobName)
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

type Output struct {
	Name       string `json:"name"`
	DockerFile string `json:"dockerfile"`
	Status     string `json:"status"`
}

func init() {
	rootCmd.AddCommand(runCmd)
	rootCmd.AddCommand(readDockerFile)
}

func outputJSON(o Output) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(o)
}

func runJob(jobName string) {
	steps := []string{
		"Checking out repository",
		"Installing dependencies",
		"Building project",
		"Running tests",
		"Job completed",
	}

	for i := range steps {
		status := "running"
		if i == len(steps)-1 {
			status = "success"
		}

		outputJSON(Output{
			Name:       "Test",
			DockerFile: "/tmp/dockerfile",
			Status:     status,
		})
		time.Sleep(500 * time.Millisecond)
	}
}
