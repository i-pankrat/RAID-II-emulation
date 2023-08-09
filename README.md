# About

A simple tutorial project emulating the operation of RAID II disk system. The project is made for experience with the Rust language.

In the current implementation disk is an abstraction over array, file is an abstraction over part of array, respectively writing to file or disk is an abstraction over writing to array.

## Limitations:
- You cannot write to a file after it has been created
- The file name must be a whole word without spaces
- You cannot delete a file

# How to use

``` shell
cargo run
```

# Available commands
- write (restore file after a single corruption)
- read
- corrup (after more than 2 damages the behavior is undefined)
- exit