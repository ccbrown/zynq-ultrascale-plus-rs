# zynq-ultrascale-plus-rs

This crate contains high-ish, mostly safe code for interfacing with Zynq UltraScale+ devices.

## Using

This crate is a library. It's meant to be used for your existing Rust codebase. If you aren't already using Rust with your Zynq UltraScale+ device, the easiest way to get started is to create a new staticlib Rust crate with C exports that you call from your C code.

## References

- The specs which this crate is based on can be found [here](https://www.xilinx.com/htmldocs/registers/ug1087/ug1087-zynq-ultrascale-registers.html).
- For a C "hello world" on a Zynq UltraScale+ using Vivado, Vitis, and a Kria SOM, see [this bare-metal workflow example](https://xilinx.github.io/kria-apps-docs/creating_applications/2022.1/build/html/docs/baremetal.html). Once you're up and running with C, you can link Rust libraries targeting aarch64-unknown-none in the usual way.
