# zynq-ultrascale-plus

## Testing With QEMU

This crate has unit tests that can be run using QEMU.

These prerequisites must be in your PATH:

- `qemu-system-aarch64` from [Xilinx/qemu](https://github.com/Xilinx/qemu).
- `aarch64-none-elf-gcc` and friends from the [ARM GNU Toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads).

You must have the `aarch64-unknown-none` target installed with the nightly toolchain.

Once the prerequisites are met, you can simply run:

```
cargo +nightly test
```

Additional arguments will be forwarded to QEMU. So for example, to see interrupt logging, you can use:

```
cargo +nightly test -- -d int
```

Or to capture network traffic:

```
cargo +nightly test -- -object filter-dump,id=f1,netdev=net0,file=dump.pcap
```

## Testing With Real Hardware

This crate's unit tests can also be run with real hardware.

You must have the `aarch64-unknown-none` target installed with the nightly toolchain, you must have Xilinx Vitis installed with `xsct` in your path, and you must have a device connected via USB.

Once the prerequisites are met, you can run:

```
TEST_RUNNER=jtag cargo +nightly test
```

That command will run the tests using JTAG boot.

You'll need to inspect the output via UART. Cargo will not wait for the tests or return any error codes.

Some of the tests must make assumptions about the available peripherals, such as ethernet PHY. These tests are tailored to the Kria KV260 starter kit and may fail or hang on other hardware. If needed, you can skip tests on real hardware by replacing the `#[test]` attribute with `#[test(qemu_only)]`. Conversely, if you want to run a test only on hardware, you can use `#[test(hw_only)]`.
