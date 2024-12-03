# Rust on SERV Experiments

Install some tools.

```shell
cargo install cargo-binutils
rustup component add llvm-tools
```

Build release and copy elf to binary.

```shell
cargo objcopy --release -- -O binary blinky.bin
```

Generate a memory file using `makehex.py` from serv.

```shell
python3 .serv/sw/makehex.py blinky-serv/blinky.bin > blinky-serv.hex
```

Build / run in testbench.

```shell
fusesoc run --target=verilator_tb servant --firmware=blinky-serv.hex --memsize=8192 --vcd true --compressed=1
```

Build / run on hardware.

```shell
fusesoc run --target=te0802 servant --firmware=blinky-serv.hex --compressed=1
```
