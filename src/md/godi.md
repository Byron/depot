`godi` stands for "go data integrity" and is a commandline utility to generate signature files from a directory tree. This allows to re-check the tree for consistency, and thus verify the data is intact. This is especially useful if data is retrieved from unreliable media, and copied to another storage device.

As it is very common to verify copy operations, `godi` is able to copy files in the moment is hashes them, optionally verifying the destination after it was copied.

## Examples

```bash
# Generate a signature for all files in directory tree/
godi seal tree/

# results in godi-seal.xml file
```


TODO: 
* nprocs - specify how many parallel gather routines there are
* abort-on-error - if False, we continue as long as possible, otherwise we abort and interrupt all currently running procedures
* log-mode - either off, or verbose, and in future, maybe even a binary one which provides a whole lot of additional information