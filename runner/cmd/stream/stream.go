// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package stream

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"os"

	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

type StatsOutput struct {
	ContainerID string  `json:"container_id"`
	CPUPercent  float64 `json:"cpu_percent"`
	MemUsage    uint64  `json:"mem_usage"`
	MemLimit    uint64  `json:"mem_limit"`
	MemPercent  float64 `json:"mem_percent"`
	Error       string  `json:"error,omitempty"`
}

type ContainerStats struct {
	ID       string `json:"id"`
	CPUStats struct {
		CPUUsage struct {
			TotalUsage uint64 `json:"total_usage"`
		} `json:"cpu_usage"`
		SystemCPUUsage uint64 `json:"system_cpu_usage"`
		OnlineCPUs     uint64 `json:"online_cpus"`
	} `json:"cpu_stats"`
	PreCPUStats struct {
		CPUUsage struct {
			TotalUsage uint64 `json:"total_usage"`
		} `json:"cpu_usage"`
		SystemCPUUsage uint64 `json:"system_cpu_usage"`
	} `json:"precpu_stats"`
	MemoryStats struct {
		Usage uint64 `json:"usage"`
		Limit uint64 `json:"limit"`
	} `json:"memory_stats"`
}

var StreamCmd = &cobra.Command{
	Use:   "stream [container_id]",
	Short: "Stream container stats",
	Long:  `Stream real-time container statistics including CPU and memory usage`,
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		containerID := args[0]
		streamStats(containerID)
	},
}

func outputJSON(result StatsOutput) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(result)
}

func streamStats(containerID string) {
	ctx := context.Background()

	cli, err := client.New(client.FromEnv)
	if err != nil {
		outputJSON(StatsOutput{
			ContainerID: containerID,
			Error:       fmt.Sprintf("failed to create docker client: %v", err),
		})
		return
	}
	defer cli.Close()

	stats, err := cli.ContainerStats(ctx, containerID, client.ContainerStatsOptions{Stream: true})
	if err != nil {
		outputJSON(StatsOutput{
			ContainerID: containerID,
			Error:       fmt.Sprintf("failed to get container stats: %v", err),
		})
		return
	}
	defer stats.Body.Close()

	decoder := json.NewDecoder(stats.Body)
	for {
		var s ContainerStats
		if err := decoder.Decode(&s); err != nil {
			if err == io.EOF {
				break
			}
			outputJSON(StatsOutput{
				ContainerID: containerID,
				Error:       fmt.Sprintf("failed to decode stats: %v", err),
			})
			return
		}

		cpuDelta := float64(s.CPUStats.CPUUsage.TotalUsage - s.PreCPUStats.CPUUsage.TotalUsage)
		systemDelta := float64(s.CPUStats.SystemCPUUsage - s.PreCPUStats.SystemCPUUsage)
		cpuPercent := 0.0
		if systemDelta > 0 && s.CPUStats.OnlineCPUs > 0 {
			cpuPercent = (cpuDelta / systemDelta) * float64(s.CPUStats.OnlineCPUs) * 100.0
		} else if systemDelta > 0 {
			cpuPercent = (cpuDelta / systemDelta) * 100.0
		}

		memPercent := 0.0
		if s.MemoryStats.Limit > 0 {
			memPercent = (float64(s.MemoryStats.Usage) / float64(s.MemoryStats.Limit)) * 100.0
		}

		outputJSON(StatsOutput{
			ContainerID: containerID,
			CPUPercent:  cpuPercent,
			MemUsage:    s.MemoryStats.Usage,
			MemLimit:    s.MemoryStats.Limit,
			MemPercent:  memPercent,
		})
	}
}
