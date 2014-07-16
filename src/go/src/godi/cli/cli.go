package cli

import (
	"errors"
	"flag"
	"fmt"
	"godi/seal"
	"os"
)

const usage = "godi {seal} [--help] args"

type CLISubCommand interface {
	SetUnparsedArgs(args []string) error
}

// Return a string representing detailed usage information, possibly based on the given parser
func HelpString(parser *flag.FlagSet) string {
	return usage + "\nTODO: LONG HELP"
}

// Parse the given arguments, which are assuming to NOT contain the invoked executable as first argument
// or use os.Args if there are no arguments given.
// We will parse subcommands ourselves, and then return one of the *Args types to indicate which subcomamnd
// was actually chosen.
// If there was an error parsing the arguments, it's error string will be usage information or what the problem
// was, useful for the end-user.
// If there was an error, options will be nil
// The interface return value can also be a string representing a detailed help string
func ParseArgs(args ...string) (interface{}, error) {
	if len(args) == 0 {
		args = append(args, os.Args[1:]...)
	}

	if len(args) < 1 {
		return nil, errors.New(usage)
	}

	// Parse based on subcommand
	var parser *flag.FlagSet
	var command CLISubCommand
	var helpFlag = false
	const helpUsage = "Prints detailed help"

	switch cmd := args[0]; cmd {
	case seal.Name:
		parser = flag.NewFlagSet(seal.Name, flag.ContinueOnError)
		parser.BoolVar(&helpFlag, "help", helpFlag, helpUsage)
		cmd := seal.SealCommand{}
		command = &cmd
	default:
		return nil, fmt.Errorf("Invalid subcommand: %s\n%s", cmd, usage)
	}

	if parser == nil || command == nil {
		panic("Should have a parser and command set by now")
	}

	if err := parser.Parse(args[1:]); err != nil {
		return nil, errors.New(usage + "\n" + err.Error())
	}

	if helpFlag {
		return HelpString(parser), nil
	}

	if err := command.SetUnparsedArgs(parser.Args()); err != nil {
		return nil, err
	}
	return command, nil
}
