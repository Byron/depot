package godi

import "sync"

// A struct holding information about a task, including
type FileInfo struct {
	// path to file to handle
	Path string

	// size of file
	Size int64

	// hash of file
	Sha1 []byte
}

type Result interface {

	// Return a string indicating the result, which can can also state an error
	Info() string

	// Return an error instance indicating what exactly when wrong
	Error() error
}

type Runner interface {

	// Return maximum amount of processes we can handle.
	// It's also based on our options, and no more than MaxProcs() go routines shouldbe started for Gather
	MaxProcs() uint

	// Launches a go-routine which fills the returned FileInfo channel
	// Must close FileInfo channel when done
	// Must listen for SIGTERM|SIGINT signals and abort if received
	// May report errrors or information about the progress through generateResult, which must be closed when done. Return nothing
	// if there is nothing to report
	// Must listen on done and return asap
	Generate(done <-chan bool) (files <-chan FileInfo, generateResult <-chan Result)

	// Will be launched as go routine and perform whichever operation on the FileInfo received from input channel
	// Produces one result per input FileInfo and returns it in the given results channel
	// Must listen for SIGTERM|SIGINT signals
	// Use the wait group to mark when done
	// Must listen on done and return asap
	Gather(files <-chan FileInfo, results chan<- Result, wg *sync.WaitGroup, done <-chan bool)

	// Accumulate the result channel and produce whatever you have to produce from the result of the Gather steps
	// When you are done, place a single result instance into accumResult and close the channel
	Accumulate(results <-chan Result) <-chan Result
}

// type Engine struct {
// 	runner Runner
// }

// func NewEngine() {

// }
