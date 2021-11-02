# RISC-V CLI Decoder

Simple CLI tool for decoding RISCV instructions. Note that this tool does **not** support all instructions (unsupported: JType, CsrIType and System instructions).

## Dependencies

- [Rust/Cargo](https://www.rust-lang.org/tools/install)

## Usage

Using cargo:

```bash
cargo run -- -h "0x01e60f33" 
```

```bash
cargo run -- -b "74039" 
```

From terminal:

```bash
./riscv-cli-decoder -h "0x01e60f33"
```
