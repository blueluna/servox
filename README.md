# Rust on SERV Experiments

Install some tools.

```shell
$ rustup component add llvm-tools
```

## Build HEX files

Build release and generate hex memory file for all binaries

```shell
$ cargo xtask build
```

Build release and generate hex memory file for a specific binary.

```shell
$ cargo xtask build <binary>
```

For more information on development see, [servox/README.md](servox/README.md).

## Blinky

Build / run in testbench.

```shell
$ fusesoc run --target=verilator_tb servant --firmware=servox/target/riscv32im-unknown-none-elf/release/blinky.hex
```

Build / run on hardware.

```shell
$ fusesoc run --target=te0802 servant --firmware=servox/target/riscv32im-unknown-none-elf/release/blinky.hex
```

## Hello

Build / run in testbench.

```shell
$ fusesoc run --target=verilator_tb servant --firmware=servox/target/riscv32im-unknown-none-elf/release/hello.hex --uart_baudrate=57600
```

Build / run on hardware.

```shell
$ fusesoc run --target=te0802 servant --firmware=servox/target/riscv32im-unknown-none-elf/release/hello.hex
```

Connect to the target at 115200 baud rate.

```shell
$ tio --mute --baud 115200 /dev/ttyUSB0
Hello, I'm Servant!
```
