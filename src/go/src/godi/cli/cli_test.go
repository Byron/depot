package cli_test

import (
	"godi/cli"
	"godi/seal"
	"testing"
)

func TestParsing(t *testing.T) {
	if _, err := cli.ParseArgs("invalid_subcommand"); err == nil {
		t.Error("Shouldn't parse invalid_subcommand")
	} else {
		t.Log(err.Error())
	}

	cmds := []string{seal.Name}
	for _, cmd := range cmds {
		if res, err := cli.ParseArgs(cmd, "--help"); err != nil {
			t.Error("--help must exist in every subcommand")
		} else if str, ok := res.(string); !ok {
			t.Errorf("Didn't see string return value: %#v %v", res, err)
		} else {
			t.Log(str)
		}
	}
}

func TestSealParsing(t *testing.T) {
	sealcmd := func(args ...string) (interface{}, error) {
		nargs := make([]string, len(args)+1)
		nargs[0] = "seal"
		copy(nargs[1:], args)
		return cli.ParseArgs(nargs...)
	}

	if args, err := sealcmd("foo", "bar"); err != nil {
		t.Errorf("seal should't fail if directory can't be read - it's part of the sanitization: %v", err)
	} else if args == nil {
		t.Error("no error, yet no args")
	} else if sealcmd, ok := args.(*seal.SealCommand); !ok {
		t.Errorf("Didn't get SealCommand, but %#v", args)
	} else if len(sealcmd.Trees) != 2 {
		t.Error("Didn't parse exactly 2 Trees")
	}
}
