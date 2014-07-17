package seal

import (
	"errors"
	"os"
	"strings"
)

const Name = "seal"

// A type representing all arguments required to drive a Seal operation
type SealCommand struct {

	// One or more trees to seal
	Trees []string
}

func (s *SealCommand) SetUnparsedArgs(args []string) error {
	s.Trees = args
	return nil
}

func (s *SealCommand) SanitizeArgs() (err error) {
	if len(s.Trees) == 0 {
		return errors.New("Please provide at least one tree to work on")
	}

	invalidTrees := make([]string, 0, len(s.Trees))
	for _, tree := range s.Trees {
		if _, err := os.Stat(tree); err != nil {
			invalidTrees = append(invalidTrees, tree)
		}
	}

	if len(invalidTrees) > 0 {
		return errors.New("Coulnd't read at least one of the given trees to verify: " + strings.Join(invalidTrees, ", "))
	}

	return err
}
