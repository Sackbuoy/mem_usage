# mem_usage

Simple tool for getting the maximum memory usage of a given command

# To build
```
cargo build --release
```
Creates an executable named `mem` in the `target/release/` directory

# Usage
```
mem <command>
```
Will print all of the output of the given command, as well as the time and max memory use

# Example
```
mem find / -iname "test"
```
