# RISC-V CLI Decoder

Simple CLI tool for decoding RISCV instructions. Note that this tool does **not** support all instructions (unsupported: JType, CsrIType and System instructions).

## Dependencies

- [Rust/Cargo](https://www.rust-lang.org/tools/install)

## Usage

Using cargo:

```bash
cargo run -- "0x01e60f33" 
```

From terminal:

```bash
./riscv-cli-decoder "0x01e60f33"
```
