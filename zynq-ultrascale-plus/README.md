# zynq-ultrascale-plus

## Testing

This crate has unit tests that can be run using QEMU.

These prerequisites must be in your PATH:

- `qemu-system-aarch64` from [Xilinx/qemu](https://github.com/Xilinx/qemu).
- `aarch64-none-elf-gcc` and friends from the [ARM GNU Toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads).

You must have the `aarch64-unknown-none` target installed with the nightly toolchain.

Once the prerequisites are met, you can simply:

```
cargo +nightly test
```

Additional arguments will be forwarded to QEMU. So for example, to see interrupt logging, you can use:

```
cargo +nightly test -- -d int
```
