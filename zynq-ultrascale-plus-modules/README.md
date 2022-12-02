# zynq-ultrascale-plus-modules

This crate contains [tock-registers](https://crates.io/crates/tock-registers) definitions generated automatically from the [Zynq UltraScale+ Devices Register Reference](https://www.xilinx.com/htmldocs/registers/ug1087/ug1087-zynq-ultrascale-registers.html).

## Generating

If you'd like to regenerate this crate, it's a two-step process:

- Run [scripts/scrape.py](./scripts/scrape.py) to collect all of the data from the reference into a JSON file.
- Run [scripts/generate.py](./scripts/generate.py) to regenerate the Rust code.
