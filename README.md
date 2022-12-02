# zynq-ultrascale-plus-rs

This repo contains two crates:

- zynq-ultrascale-plus contains high-ish level, mostly safe code for interfacing with Zynq UltraScale+ devices.
- zynq-ultrascale-plus-modules contains [tock-registers](https://crates.io/crates/tock-registers) definitions generated automatically from the [Zynq UltraScale+ Devices Register Reference](https://www.xilinx.com/htmldocs/registers/ug1087/ug1087-zynq-ultrascale-registers.html).

## Getting Started

If you're already using Rust with your Zynq UltraScale+ device, you can use this crate like any other library.

If you're using C with your Zynq UltraScale+ device, you should create a new staticlib Rust crate with C exports that you can call from your C code.

If you're building a new project from scratch, you should start with a C "hello world" like described in [this bare-metal example for the Kria SOM](https://xilinx.github.io/kria-apps-docs/creating_applications/2022.1/build/html/docs/baremetal.html). Then delete all the C files from the project, create a new staticlib Rust crate with a lib.rs that looks like the following, and link it to your project.

```rust
#![no_std]

#[no_mangle]
pub extern "C" fn main() {
    unsafe {
        zynq_ultrascale_plus::Device::new().uart1().send_bytes("hello from rust!\n\r");
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    // todo: dump the info via uart or reboot or something
    loop {}
}
```

## Tips for Working with Bare Metal

### Using an Allocator

If you want to use `alloc`, you can use [rust-osdev/linked-list-allocator](https://github.com/rust-osdev/linked-list-allocator) to add an allocator like this:

```rust
#![feature(default_alloc_error_handler)]

#[macro_use]
extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

extern "C" {
    // these are defined by the project's linker script (usually src/lscript.ld)
    static mut _heap_start: u32;
    static mut _heap_end: u32;
}

// CALL THIS BEFORE DOING ANYTHING ELSE!
pub fn init_heap() {
    unsafe {
        let heap_start_ptr = &mut _heap_start as *mut u32;
        let heap_end_ptr = &mut _heap_end as *mut u32;
        ALLOCATOR.lock().init(heap_start_ptr as *mut u8, heap_end_ptr as usize - heap_start_ptr as usize);
    }
}
```

## References

- The specs which this crate is based on can be found [here](https://www.xilinx.com/htmldocs/registers/ug1087/ug1087-zynq-ultrascale-registers.html).
