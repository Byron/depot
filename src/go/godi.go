package main

import (
	"fmt"
	"godi"
	"godi/cli"
	"os"
)

const (
	PROGRAMMING_ERROR = 255
	ARGUMENT_ERROR    = 1
	OTHER_ERROR       = 3
)

// DEBUG
const nprocs = 1

func main() {
	cmd, err := cli.ParseArgs(os.Args[1:]...)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(ARGUMENT_ERROR)
	}

	switch c := cmd.(type) {
	case string:
		{
			// Handle help printing
			fmt.Fprintln(os.Stdout, c)
			os.Exit(ARGUMENT_ERROR)
		}
	case cli.SubCommand:
		if err := c.SanitizeArgs(); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(ARGUMENT_ERROR)
		}

		if runner, ok := cmd.(godi.Runner); !ok {
			fmt.Fprintln(os.Stderr, "Didn't get Runner interface from cli parser")
			os.Exit(PROGRAMMING_ERROR)
		} else {
			godi.StartEngine(runner, nprocs)
		}
	default:
		fmt.Fprintf(os.Stderr, "Invalid command type returned - it didn't support the runner interfacea: %#v", cmd)
		os.Exit(PROGRAMMING_ERROR)
	}
}
