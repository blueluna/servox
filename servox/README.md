# Rust on SERV Experiments

## Building without xtask

To build without using xtask, start by installing `cargo-binutils`.

```shell
$ cargo install cargo-binutils
```

Build release and copy elf to binary.

```shell
$ cargo objcopy --release --bin blinky -- -O binary blinky.bin
```

Generate a hex memory file using `makehex.py` from the SERV project.

```shell
$ python3 ./serv/sw/makehex.py servox/blinky.bin > blinky.hex
```

## Notes

### UART timing

To get the correct timing bitbanging UART, loop unrolling has been "disabled" with the compilter flag `llvm-args=-unroll-threshold=10`.

### RV32 compression

It is possible to use the `riscv32imc` toolchain if SERV is built to support it, which is optional.
