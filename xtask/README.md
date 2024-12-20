# Xtask tools

## Build

Build as release and produce a hex memory file.

```shell
$ cargo xtask build <binary>
```

Performs the equivalent of,

 ```shell
 cargo build --release (--bins | --bin <binary>)
 objcopy -O binary <elf> <bin>
 python3 ./serv/sw/makehex.py <bin> > <hex>
 ```
