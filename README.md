# Tern
This is a poorly implemented VM with a MIPS-style architecture based on [balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary) arithmetic. It operates on 6-trit trytes, 12-trit halfwords, and 24-trit words, and has built-in support for text encoded in [UTF-6t](src/text.rs), the ternary analogue of UTF-8. This project is not finished, as the intention is to eventually replace it with a much better design, informed by the many lessons I've learned along the way about Rust programming and VM/CPU design.

## Improvements for the next version
- Use as little unsafe code as possible
- Address trytes instead of trits
- Operate on trytes as 16-bit integers instead of individual trits as bytes
- Encode every instruction as a single tryte
- Match the classic MIPS instruction set more closely
- Add a `div` instruction for integer division and modulus
- Use a real parser for the assembler instead of a pile of regexes
- Implement UTF-6t functionality in assembly code instead of opaque syscalls

## Usage
There are a few binaries in the project, but the primary one is `run`, which allows you to execute the assembly files in the `programs` directory: `cargo run --bin run programs/hash.tasm`
