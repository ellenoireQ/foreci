// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package images

import (
	"encoding/json"
	"os"
	"os/exec"
	"strings"

	"github.com/spf13/cobra"
)

type DockerImage struct {
	Repository string `json:"repository"`
	Tag        string `json:"tag"`
	ImageID    string `json:"image_id"`
	Created    string `json:"created"`
	Size       string `json:"size"`
}

var ImagesCmd = &cobra.Command{
	Use:   "images",
	Short: "List Docker images",
	Long:  `List all Docker images from the local Docker daemon`,
	Run: func(cmd *cobra.Command, args []string) {
		listImages()
	},
}

func outputJSON(img DockerImage) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(img)
}

func listImages() {
	cmd := exec.Command("docker", "images", "--format", "{{.Repository}}\t{{.Tag}}\t{{.ID}}\t{{.CreatedSince}}\t{{.Size}}")
	output, err := cmd.Output()
	if err != nil {
		return
	}

	lines := strings.Split(string(output), "\n")
	for _, line := range lines {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) >= 5 {
			outputJSON(DockerImage{
				Repository: parts[0],
				Tag:        parts[1],
				ImageID:    parts[2],
				Created:    parts[3],
				Size:       parts[4],
			})
		}
	}
}
