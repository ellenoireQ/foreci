// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

package create

import (
	"archive/tar"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/netip"
	"os"
	"path/filepath"
	"strings"

	"github.com/moby/moby/api/types/container"
	"github.com/moby/moby/api/types/network"
	"github.com/moby/moby/client"
	"github.com/spf13/cobra"
)

type CreateResult struct {
	ContainerID   string `json:"container_id"`
	ContainerName string `json:"container_name"`
	Status        string `json:"status"`
	Error         string `json:"error,omitempty"`
}

var (
	imageName     string
	containerName string
	hostname      string
	ports         string
	environment   string
	volumes       string
	networks      string
	restart       string
	autoStart     bool
	buildContext  string
)

var CreateCmd = &cobra.Command{
	Use:   "create",
	Short: "Create a Docker container",
	Long:  `Create a Docker container from docker-compose configuration`,
	Run: func(cmd *cobra.Command, args []string) {
		createContainer()
	},
}

func init() {
	CreateCmd.Flags().StringVarP(&imageName, "image", "i", "", "Image name (required)")
	CreateCmd.Flags().StringVarP(&containerName, "name", "n", "", "Container name")
	CreateCmd.Flags().StringVarP(&hostname, "hostname", "H", "", "Container hostname")
	CreateCmd.Flags().StringVarP(&ports, "ports", "p", "", "Port mappings (comma-separated, e.g. '8080:80,443:443')")
	CreateCmd.Flags().StringVarP(&environment, "env", "e", "", "Environment variables (comma-separated, e.g. 'FOO=bar,BAZ=qux')")
	CreateCmd.Flags().StringVarP(&volumes, "volumes", "v", "", "Volume mounts (comma-separated, e.g. '/host:/container,/data:/data')")
	CreateCmd.Flags().StringVarP(&networks, "networks", "N", "", "Networks (comma-separated)")
	CreateCmd.Flags().StringVarP(&restart, "restart", "r", "", "Restart policy (no, always, unless-stopped, on-failure)")
	CreateCmd.Flags().BoolVarP(&autoStart, "start", "s", false, "Auto-start container after creation")
	CreateCmd.Flags().StringVarP(&buildContext, "build-context", "b", "", "Build context path (for building image from Dockerfile)")
	CreateCmd.MarkFlagRequired("image")
}

func outputJSON(result CreateResult) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(result)
}

func parsePortBindings(portsStr string) (network.PortSet, network.PortMap) {
	exposedPorts := make(network.PortSet)
	portBindings := make(network.PortMap)

	if portsStr == "" {
		return exposedPorts, portBindings
	}

	portPairs := strings.Split(portsStr, ",")
	for _, pair := range portPairs {
		pair = strings.TrimSpace(pair)
		if pair == "" {
			continue
		}

		parts := strings.Split(pair, ":")
		var hostPort, containerPort string

		if len(parts) == 2 {
			hostPort = parts[0]
			containerPort = parts[1]
		} else if len(parts) == 1 {
			hostPort = parts[0]
			containerPort = parts[0]
		} else {
			continue
		}

		port, err := network.ParsePort(containerPort + "/tcp")
		if err != nil {
			continue
		}
		exposedPorts[port] = struct{}{}
		portBindings[port] = []network.PortBinding{
			{HostIP: netip.MustParseAddr("0.0.0.0"), HostPort: hostPort},
		}
	}

	return exposedPorts, portBindings
}

func parseEnvVars(envStr string) []string {
	if envStr == "" {
		return nil
	}

	envVars := strings.Split(envStr, ",")
	var result []string
	for _, env := range envVars {
		env = strings.TrimSpace(env)
		if env != "" {
			result = append(result, env)
		}
	}
	return result
}

func parseVolumes(volumesStr string) []string {
	if volumesStr == "" {
		return nil
	}

	vols := strings.Split(volumesStr, ",")
	var result []string
	for _, vol := range vols {
		vol = strings.TrimSpace(vol)
		if vol != "" {
			result = append(result, vol)
		}
	}
	return result
}

func getRestartPolicy(policyStr string) container.RestartPolicy {
	switch policyStr {
	case "always":
		return container.RestartPolicy{Name: container.RestartPolicyAlways}
	case "unless-stopped":
		return container.RestartPolicy{Name: container.RestartPolicyUnlessStopped}
	case "on-failure":
		return container.RestartPolicy{Name: container.RestartPolicyOnFailure}
	default:
		return container.RestartPolicy{Name: container.RestartPolicyDisabled}
	}
}

func imageExists(cli *client.Client, imageName string) bool {
	ctx := context.Background()
	result, err := cli.ImageList(ctx, client.ImageListOptions{})
	if err != nil {
		return false
	}
	for _, img := range result.Items {
		for _, tag := range img.RepoTags {
			if tag == imageName {
				return true
			}
		}
	}
	return false
}

func buildImage(cli *client.Client, imageName string, contextPath string) error {
	outputPullProgress(PullProgress{
		Image:  imageName,
		Status: "building",
	})

	tarReader, err := createTarFromDir(contextPath)
	if err != nil {
		outputPullProgress(PullProgress{
			Image:  imageName,
			Status: "error",
			Error:  fmt.Sprintf("failed to create build context: %v", err),
		})
		return err
	}

	buildOptions := client.ImageBuildOptions{
		Tags:       []string{imageName},
		Dockerfile: "Dockerfile",
		Remove:     true,
	}

	resp, err := cli.ImageBuild(context.Background(), tarReader, buildOptions)
	if err != nil {
		outputPullProgress(PullProgress{
			Image:  imageName,
			Status: "error",
			Error:  fmt.Sprintf("failed to build image: %v", err),
		})
		return err
	}
	defer resp.Body.Close()

	decoder := json.NewDecoder(resp.Body)
	for {
		var progress map[string]interface{}
		if err := decoder.Decode(&progress); err != nil {
			if err == io.EOF {
				break
			}
			break
		}
		if stream, ok := progress["stream"].(string); ok {
			outputPullProgress(PullProgress{
				Image:    imageName,
				Status:   "building",
				Progress: strings.TrimSpace(stream),
			})
		}
		if errMsg, ok := progress["error"].(string); ok {
			outputPullProgress(PullProgress{
				Image:  imageName,
				Status: "error",
				Error:  errMsg,
			})
			return fmt.Errorf("build error: %s", errMsg)
		}
	}

	outputPullProgress(PullProgress{
		Image:  imageName,
		Status: "completed",
	})

	return nil
}

func createTarFromDir(srcDir string) (io.Reader, error) {
	pr, pw := io.Pipe()

	go func() {
		tw := tar.NewWriter(pw)
		defer tw.Close()
		defer pw.Close()

		err := filepath.Walk(srcDir, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return err
			}

			relPath, err := filepath.Rel(srcDir, path)
			if err != nil {
				return err
			}

			if info.IsDir() && strings.HasPrefix(info.Name(), ".") && info.Name() != "." {
				return filepath.SkipDir
			}

			header, err := tar.FileInfoHeader(info, "")
			if err != nil {
				return err
			}
			header.Name = relPath

			if err := tw.WriteHeader(header); err != nil {
				return err
			}

			if !info.IsDir() {
				file, err := os.Open(path)
				if err != nil {
					return err
				}
				defer file.Close()
				_, err = io.Copy(tw, file)
				return err
			}
			return nil
		})

		if err != nil {
			pw.CloseWithError(err)
		}
	}()

	return pr, nil
}

func pullImage(cli *client.Client, imageName string) error {
	// Skip if image already exists
	if imageExists(cli, imageName) {
		outputPullProgress(PullProgress{
			Image:  imageName,
			Status: "exists",
		})
		return nil
	}

	outputPullProgress(PullProgress{
		Image:  imageName,
		Status: "pulling",
	})

	out, err := cli.ImagePull(context.Background(), imageName, client.ImagePullOptions{})
	if err != nil {
		outputPullProgress(PullProgress{
			Image:  imageName,
			Status: "error",
			Error:  err.Error(),
		})
		return err
	}
	defer out.Close()

	// Read and parse progress from Docker
	decoder := json.NewDecoder(out)
	for {
		var progress map[string]interface{}
		if err := decoder.Decode(&progress); err != nil {
			break
		}
		if status, ok := progress["status"].(string); ok {
			outputPullProgress(PullProgress{
				Image:    imageName,
				Status:   "downloading",
				Progress: status,
			})
		}
	}

	outputPullProgress(PullProgress{
		Image:  imageName,
		Status: "completed",
	})

	return nil
}

type PullProgress struct {
	Image    string `json:"image"`
	Status   string `json:"status"`
	Progress string `json:"progress,omitempty"`
	Error    string `json:"error,omitempty"`
}

func outputPullProgress(p PullProgress) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(p)
}
func createContainer() {
	ctx := context.Background()

	cli, err := client.New(client.FromEnv)
	if err != nil {
		outputJSON(CreateResult{
			ContainerName: containerName,
			Status:        "error",
			Error:         fmt.Sprintf("failed to create docker client: %v", err),
		})
		return
	}
	defer cli.Close()

	exposedPorts, portBindings := parsePortBindings(ports)
	envVars := parseEnvVars(environment)
	binds := parseVolumes(volumes)
	restartPolicy := getRestartPolicy(restart)

	config := &container.Config{
		Image:        imageName,
		Hostname:     hostname,
		Env:          envVars,
		ExposedPorts: exposedPorts,
	}

	hostConfig := &container.HostConfig{
		PortBindings:  portBindings,
		Binds:         binds,
		RestartPolicy: restartPolicy,
	}

	networkConfig := &network.NetworkingConfig{}

	opts := client.ContainerCreateOptions{
		Config:           config,
		HostConfig:       hostConfig,
		NetworkingConfig: networkConfig,
		Name:             containerName,
	}

	// Build or pull image before creating container
	if buildContext != "" {
		if err := buildImage(cli, imageName, buildContext); err != nil {
			outputJSON(CreateResult{
				ContainerName: containerName,
				Status:        "error",
				Error:         fmt.Sprintf("failed to build image: %v", err),
			})
			return
		}
	} else {
		pullImage(cli, imageName)
	}

	resp, err := cli.ContainerCreate(ctx, opts)
	if err != nil {
		outputJSON(CreateResult{
			ContainerName: containerName,
			Status:        "error",
			Error:         fmt.Sprintf("failed to create container: %v", err),
		})
		return
	}

	result := CreateResult{
		ContainerID:   resp.ID,
		ContainerName: containerName,
		Status:        "created",
	}

	if autoStart {
		_, err = cli.ContainerStart(ctx, resp.ID, client.ContainerStartOptions{})
		if err != nil {
			result.Status = "created"
			result.Error = fmt.Sprintf("container created but failed to start: %v", err)
		} else {
			result.Status = "running"
		}
	}

	outputJSON(result)
}
