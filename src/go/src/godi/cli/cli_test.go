package cli_test

import (
	"fmt"
	"godi"
	"godi/cli"
	"godi/seal"
	"io/ioutil"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"sync"
	"syscall"
	"testing"
	"time"
)

// Create a new file at given path and size, without possibly required intermediate directories
func makeFileOrPanic(path string, size int) string {
	f, err := os.Create(path)
	if err != nil {
		panic(err)
	}

	if size != 0 {
		b := [1]byte{0}
		f.WriteAt(b[:], int64(size-1))
	}

	return path
}

// Create a dataset for testing and return the newly created directory
func makeDatasetOrPanic() (string, string) {
	base, err := ioutil.TempDir("", "dataset")
	if err != nil {
		panic(err)
	}

	makeFileOrPanic(filepath.Join(base, "1mb.ext"), 1024*1024)
	makeFileOrPanic(filepath.Join(base, "somebytes_noext"), 313)

	subdir := filepath.Join(base, "subdir")
	if err := os.Mkdir(subdir, 0777); err != nil {
		panic(err)
	}
	makeFileOrPanic(filepath.Join(subdir, "biggie.foo"), 1024*1024+5123)
	makeFileOrPanic(filepath.Join(subdir, "smallie.blah"), 123)
	subdir = filepath.Join(base, "nothing", "stillnothing", "ünicod€")
	if err := os.MkdirAll(subdir, 0777); err != nil {
		panic(err)
	}

	file := makeFileOrPanic(filepath.Join(subdir, "somefile.ext"), 12345)
	return base, file
}

// Delete the given tree entirely. Should only be used in conjunction with makeDataset
// Panics if something is wrong
// Will only do the work if we are not already in panic
func rmTree(tree string) {
	if len(tree) == 0 {
		panic("Invalid tree given")
	}
	res := recover()
	if res != nil {
		fmt.Fprintf(os.Stderr, "Keeping tree for debugging at '%s'", tree)
		panic(res)
	}
	if err := os.RemoveAll(tree); err != nil {
		panic(err)
	}
}

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

	sealcmdChecked := func(args ...string) *seal.SealCommand {
		if res, err := sealcmd(args...); err != nil {
			t.Error("Parsing shouldn't fail with no arguments")
		} else if cmd, ok := res.(*seal.SealCommand); !ok {
			t.Errorf("invalid return type: %v", res)
		} else {
			return cmd
		}
		panic("Shouldn't be here")
	}

	if res, err := sealcmd("foo", "bar"); err != nil {
		t.Errorf("seal should't fail if directory can't be read - it's part of the sanitization: %v", err)
	} else if res == nil {
		t.Error("no error, yet no args")
	} else if scmd, ok := res.(*seal.SealCommand); !ok {
		t.Errorf("Didn't get SealCommand, but %#v", res)
	} else if len(scmd.Trees) != 2 {
		t.Error("Didn't parse exactly 2 Trees")
	} else if err := scmd.SanitizeArgs(); err == nil {
		t.Error("Expected that all directories are invalid")
	} else if !strings.Contains(err.Error(), "foo, bar") {
		t.Errorf("Error string unexpected: %v", err)
	} else {
		t.Log(err)
	}

	cmd := sealcmdChecked()
	if err := cmd.SanitizeArgs(); err == nil {
		t.Error("Expected error as empty trees are disallowed")
	} else {
		t.Log(err)
	}

	datasetTree, dataFile := makeDatasetOrPanic()
	defer rmTree(datasetTree)

	cmd = sealcmdChecked(dataFile)
	if err := cmd.SanitizeArgs(); err == nil {
		t.Error("Expected it to not like files as directory")
	} else {
		t.Log(err)
	}

	cmd = sealcmdChecked(datasetTree)
	if err := cmd.SanitizeArgs(); err != nil {
		t.Error("Sanitize didn't like existing tree")
	}

	var nprocs uint = 1
	if nprocs > cmd.MaxProcs() {
		t.Error("Can't do less than one process here ... ")
	}

	results := make(chan godi.Result, nprocs)
	done := make(chan bool)

	// assure we close our done channel on signal
	signals := make(chan os.Signal, 2)
	signal.Notify(signals, syscall.SIGINT, syscall.SIGTERM)
	go func() {
		<-signals
		close(done)
	}()

	files, generateResult := cmd.Generate(done)
	wg := sync.WaitGroup{}
	for i := 0; uint(i) < nprocs; i++ {
		wg.Add(1)
		cmd.Gather(files, results, &wg, done)
	}
	go func() {
		wg.Wait()
		close(results)
	}()
	accumResult := cmd.Accumulate(results)

	// Return true if we should break the loop
	resHandler := func(name string, res godi.Result) bool {
		if res == nil {
			// channel closed, have to get out
			t.Log("Channel", name, "is closed")
			return true
		}

		if res.Error() != nil {
			t.Error(res.Error())
		} else {
			t.Log(res.Info())
		}

		return false
	} // end resHandler

infinity:
	for {
		select {
		case r := <-generateResult:
			{
				if resHandler("generator", r) {
					break infinity
				}
			}
		case r := <-accumResult:
			{
				if resHandler("accumulator", r) {
					break infinity
				}
			}
		case <-time.After(5 * time.Second):
			t.Fatal("Didn't get result after timeout")
		} // select
	} // endless loop

	// if rprocs != 0 {
	// 	t.Error("nprocs - rprocs mismatch", rprocs, nprocs)
	// }

	// // godi.
	// cmd.Generate()
	// if err := cmd.Run(); err != nil {
	// 	t.Error(err)
	// }

}
