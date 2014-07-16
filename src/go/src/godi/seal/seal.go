package seal

const Name = "seal"

// A type representing all arguments required to drive a Seal operation
type SealCommand struct {

	// One or more trees to seal
	Trees []string
}

func (cmd *SealCommand) SetUnparsedArgs(args []string) error {
	cmd.Trees = args
	return nil
}
