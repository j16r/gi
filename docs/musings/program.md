# Program structure

## Command line

    gi --daemon
    gi -d

Launches the gi daemon and backgrounds it.

    gi file.gi

Loads and executes file.gi, ships it to the daemon if running.

    gi -d file.gi

## Example

For the example program (a fibonacci sequence):

On the command line:

    gi fibonacci.gi

The following sequence of instructions happen:

    require 'core'
    ns 'fibonacci'
    require './fibonacci.gi'
    call main



