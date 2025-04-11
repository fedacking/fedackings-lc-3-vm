# fedackings-lc-3-vm
A rust implementation of an lc-3 virtual machine.

Based on the following tutorial: https://www.jmeiners.com/lc3-vm/

## Included binaries

The repo comes with 23included binaries to run, stored in the binaries folder for the purposes of testing and validating the VM. The first one is `binaries/test.obj` that prints an "@\n" in the terminal and halts. The second one is `binaries/2048.obj` that runs the 2048 game in the terminal. The third one is `binaries/rogue.obj` that runs the rogue game in the terminal.

## Building and running

To build the program use the following command. The output will be done in target/debug.

```sh
make build
```

To run with one of the binaries, it has to be provided as a "path" commandline argument to make run. By default, it will run with binaries/2048.obj

```sh
make run path=rogue.obj
```

Tests can be executed with:

```sh
make test
```

## Techincal details

The VM runs entirely in memory, and doesn't implement any persistence mechanism.
