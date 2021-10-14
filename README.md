# RISC-V CLI Decoder

Simple CLI tool for decoding RISCV instructions. Note that this tool does **not** contain any error handeling or input validation and will panic on faulty input or the yet to be implemented instructions (JType, CsrIType and System instructions).

## Dependencies

- [Rust/Cargo](https://www.rust-lang.org/tools/install)

## Usage

Using cargo:

```bash
cargo run -- -d "0x01e60f33" 
```

From terminal:

```bash
./riscv-cli-decoder -d "0x01e60f33"
```
