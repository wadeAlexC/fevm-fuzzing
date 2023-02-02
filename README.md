# fevm-fuzzing

Differential fuzzing of Filecoin and Geth EVM arithmetic operations.

This project generates random input, converts the input to EVM words, and calls arithmetic operations defined in both fevm and geth.

Input and fuzzing is performed by [honggfuzz](https://github.com/rust-fuzz/honggfuzz-rs). I'd recommend skimming their docs to understand what's going on - and then read them in more detail when things break.

## Requirements

* Go, version >= 1.18
* Cargo, version >= 1.67
* Python3

Filecoin builtin actors should be checked out in a sibling repo. Also, I specifically checked out [this commit](https://github.com/filecoin-project/builtin-actors/commit/e6b24ee1fd009dbe37de7c7d0e679ed258ab085a) because shortly after, a bunch of the EVM modules were made private.

Additionally, if you want to debug crashes with lldb, you'll need more requirements. Check honggfuzz docs for more info.

## Running the fuzzer

Assuming you have the requirements listed above, you should be able to build with `cargo build` or build+run the fuzzer with `cargo hfuzz run fevm-fuzz`

## Helpful Commands

* Run fuzzer: `cargo hfuzz run fevm-fuzz`
* Inspect crashes with lldb: `cargo hfuzz run-debug fevm-fuzz hfuzz_workspace/*/*.fuzz`
* Clean fuzzer: `cargo hfuzz clean`
* Generate header file to check exported function definitions: `go tool cgo -exportheader header.h calculate.go`