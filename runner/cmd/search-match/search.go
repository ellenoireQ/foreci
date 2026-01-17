package searchmatch

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
)

type Results struct {
	Filepath string `json:"filepath"`
}

func outputJSON(o Results) {
	encoder := json.NewEncoder(os.Stdout)
	encoder.Encode(o)
}

/*
* Searching matching file with prefix
* @param SearchMatchesFile(string) -> nil, nil
*
* Output:
* {
*	 filepath: string
*	}
* */
func SearchMatchesFile(prefix string) ([]string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}

	parent := filepath.Dir(home)

	users, err := os.ReadDir(parent)
	if err != nil {
		return nil, err
	}

	var results []string

	for _, u := range users {
		if !u.IsDir() {
			continue
		}

		userHome := filepath.Join(parent, u.Name())

		filepath.WalkDir(userHome, func(path string, d os.DirEntry, err error) error {
			if err != nil {
				return nil
			}

			if d.IsDir() {
				return nil
			}

			if strings.HasPrefix(d.Name(), prefix) {
				results = append(results, filepath.Dir(path))
				return filepath.SkipDir
			}

			return nil
		})
	}
	for _, res := range results {
		outputJSON(Results{
			Filepath: res,
		})
	}
	return nil, nil
}
