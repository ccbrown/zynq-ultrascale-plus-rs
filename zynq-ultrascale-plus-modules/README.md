# zynq-ultrascale-plus-modules

This crate contains [tock-registers](https://crates.io/crates/tock-registers) definitions generated automatically from the [Zynq UltraScale+ Devices Register Reference](https://www.xilinx.com/htmldocs/registers/ug1087/ug1087-zynq-ultrascale-registers.html).

## Using

Each module is gated by a Cargo feature, and by default no features are enabled. If you enable all of the modules, the crate becomes massive and painfully slow to compile. Hence it's recommended that you enable the modules you'll be using explicitly. For example, to use the UART registers, enable the "uart" feature.

### Example

To send data via UART1, add "tock-registers" and "zynq-ultrascale-plus-modules" with the "uart" feature to your Cargo.toml.

Then, you can send bytes like so:

```rust
use tock_registers::interfaces::{Readable, Writeable};
use zynq_ultrascale_plus_modules::uart;

fn send_byte(b: u8) {
    let reg = unsafe { &mut *uart::UART1 };
    while reg.channel_sts.is_set(uart::ChannelSts::TFUL) {}
    reg.tx_rx_fifo.write(TxRxFifoW::FIFO.val(b as u32));
}
```

## Generating

If you'd like to regenerate this crate, first install the prerequisites:

- Python 3
- Requests (`pip3 install requests`)
- Beautiful Soup 4 (`pip3 install beautifulsoup4`)

From there, it's a two-step process:

- Run [scripts/scrape.py](./scripts/scrape.py) to collect all of the data from the reference into a JSON file.
- Run [scripts/generate.py](./scripts/generate.py) to regenerate the Rust code.
