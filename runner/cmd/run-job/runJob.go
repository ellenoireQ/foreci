// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package runjob

import (
	"encoding/json"
	"os"
	"time"
)

type Output struct {
	Name       string `json:"name"`
	DockerFile string `json:"dockerfile"`
	Status     string `json:"status"`
}

func outputJSON(o Output) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(o)
}

func RunJob(jobName string) {
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
