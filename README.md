# rust-go-example
Example of Calling Go Functions from Rust

The example comes from [Calling Go Functions from Other Languages](https://github.com/vladimirvivien/go-cshared-examples)

## Commands

Generate header file to check exported function definitions: `go tool cgo -exportheader header.h calculate.go`

Fuzz: `cargo hfuzz run fevm-fuzz`
Inspect crashes: `cargo hfuzz run-debug fevm-fuzz hfuzz_workspace/*/*.fuzz`
Clean fuzzer: `cargo hfuzz clean`