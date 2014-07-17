package seal

import (
	"errors"
	"flag"
	"godi"
	"math"
	"os"
	"os/signal"
	"strings"
	"sync"
	"syscall"
)

const Name = "seal"

// A type representing all arguments required to drive a Seal operation
type SealCommand struct {

	// One or more trees to seal
	Trees []string
}

// Implements information about a seal operation
type SealResult struct {
	finfo *godi.FileInfo
	msg   string
	err   error
}

func (s *SealResult) Info() string {
	if s.err != nil {
		return s.err.Error()
	}
	return s.msg
}

func (s *SealResult) Error() error {
	return s.err
}

func (s *SealCommand) SetUnparsedArgs(args []string) error {
	s.Trees = args
	return nil
}

func (s *SealCommand) MaxProcs() uint {
	return uint(math.MaxUint32)
}

func (s *SealCommand) SanitizeArgs() (err error) {
	if len(s.Trees) == 0 {
		return errors.New("Please provide at least one tree to work on")
	}

	invalidTrees := make([]string, 0, len(s.Trees))
	noTrees := make([]string, 0, len(s.Trees))
	for _, tree := range s.Trees {
		if stat, err := os.Stat(tree); err != nil {
			invalidTrees = append(invalidTrees, tree)
		} else if !stat.IsDir() {
			noTrees = append(noTrees, tree)
		}
	}

	if len(invalidTrees) > 0 {
		return errors.New("Coulnd't read at least one of the given trees to verify: " + strings.Join(invalidTrees, ", "))
	}
	if len(noTrees) > 0 {
		return errors.New("The following trees are no directory: " + strings.Join(noTrees, ", "))
	}

	return err
}

func (s *SealCommand) SetupParser(parser *flag.FlagSet) error {
	return nil
}

func (s *SealCommand) Generate() (<-chan godi.FileInfo, <-chan godi.Result) {
	files := make(chan godi.FileInfo)
	defer close(files)
	generateResult := make(chan godi.Result)
	defer close(generateResult)

	signals := make(chan os.Signal, 2)
	signal.Notify(signals, syscall.SIGINT, syscall.SIGTERM)

	go func() {
		for _, tree := range s.Trees {
			if !s.traverseFilesRecursively(files, generateResult, signals, tree) {
				// interrupted usually, or there was an error
				break
			}
		}
	}()

	return files, generateResult
}

// Traverse recursively, return false if the caller should stop traversing due to an error
func (s *SealCommand) traverseFilesRecursively(files chan<- godi.FileInfo, generateResult chan<- godi.Result, signals <-chan os.Signal, tree string) bool {

	select {
	case <-signals:
		return false
	default:
		{
			// read dir and, build file info, and recurse into subdirectories
		}
	}

	return true
}

func (s *SealCommand) Gather(files <-chan godi.FileInfo, results chan<- godi.Result, wg *sync.WaitGroup) {
	defer wg.Done()
	// for f := range files {

	// }
}

func (s *SealCommand) Accumulate(results <-chan godi.Result) <-chan godi.Result {
	accumResult := make(chan godi.Result)
	defer close(accumResult)
	return accumResult
}
