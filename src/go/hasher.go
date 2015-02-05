package main

import (
	"crypto/sha1"
	"fmt"
	"os"
	"syscall"

	"code.google.com/p/go.crypto/ssh/terminal"
)

func main() {
	hasher := sha1.New()
	b := make([]byte, 65536)

	c := 40
	len := 344
	if len%64 < 56 {
		println(56 - len%64)
	} else {
		println(64 + 56 - len%64)
	}
	fmt.Fprintln(os.Stderr, "%v ^%v", c, ^c)

	if terminal.IsTerminal(syscall.Stdin) {
		fmt.Fprintln(os.Stderr, "Stdin must not be a tty - pipe something into the process")
		os.Exit(2)
	}

	for nb, err := os.Stdin.Read(b); err == nil; nb, err = os.Stdin.Read(b) {
		hasher.Write(b[:nb])
	}

	res := hasher.Sum(nil)
	stat, _ := os.Stdin.Stat()
	fmt.Printf("%x %s\n", res, stat.Name())
}
