# Rust on SERV Experiments

Install some tools.

```shell
$ cargo install cargo-binutils
$ rustup component add llvm-tools
```

## Blinky

Build release and copy elf to binary.

```shell
$ cargo objcopy --release --bin blinky -- -O binary blinky.bin
```

Generate a memory file using `makehex.py` from serv.

```shell
$ python3 ./serv/sw/makehex.py servox/blinky.bin > blinky.hex
```

Build / run in testbench.

```shell
$ fusesoc run --target=verilator_tb servant --firmware=blinky.hex
```

Build / run on hardware.

```shell
$ fusesoc run --target=te0802 servant --firmware=blinky.hex
```

## Hello

Build release and copy elf to binary.

```shell
$ cargo objcopy --release --bin hello -- -O binary hello.bin
```

Generate a memory file using `makehex.py` from serv.

```shell
$ python3 ./serv/sw/makehex.py servox/hello.bin > hello.hex
```

Build / run in testbench.

```shell
$ fusesoc run --target=verilator_tb servant --firmware=hello.hex --uart_baudrate=57600
```

Build / run on hardware.

```shell
$ fusesoc run --target=te0802 servant --firmware=hello.hex
```

Connect to the target at 115200 baud rate.

```shell
$ tio --mute --baud 115200 /dev/ttyUSB0
Hello, I'm Servant!
```