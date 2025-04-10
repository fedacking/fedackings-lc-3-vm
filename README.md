# fedackings-lc-3-vm
A rust implementation of an lc-3 virtual machine.

Based on the following tutorial: https://www.jmeiners.com/lc3-vm/

## Included binaries

The repo comes with 2 included binaries to run, stored in the binaries folder for the purposes of testing and validating the VM. The first one is `binaries/test.obj` that prints an "@\n" in console and halting. The second one is `binaries/2048.obj` that runs the 2048 game in the console.

## Building and running

To build the program use the following command. The output will be done in target/debug.

```sh
make build
```

To run one of the binaries, it has to be provided as a commandline argument on the executable.

```sh
./target/debug/fedackings-lc-3-vm binaries/test.obj
```

Tests can be executed with:

```sh
make test
```

## Techincal details
