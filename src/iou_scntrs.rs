use tock_registers::{
    interfaces::{ReadWriteable, Readable},
    registers::{ReadOnly, ReadWrite},
};

pub struct Module {
    registers: &'static mut Registers,
}

register_structs! {
    pub Registers {
        (0x0000000000 => counter_control: ReadWrite<u32, Control::Register>),
        (0x0000000004 => counter_status: ReadOnly<u32, Status::Register>),
        (0x0000000008 => current_counter_value_lower: ReadWrite<u32>),
        (0x000000000C => current_counter_value_upper: ReadWrite<u32>),
        (0x0000000010 => _reserved),
        (0x0000000020 => base_frequency_id: ReadWrite<u32>),
        (0x0000000024 => @END),
    }
}

register_bitfields! [
    u32,
    Control [
        HDBG 1,
        EN 0,
    ],
    Status [
        HDBG 1,
    ]
];

impl Module {
    /// Creates a new system timestamp generator.
    ///
    /// # Safety
    /// `base_address` must be the base address for a system timestamp generator that is not being used elsewhere.
    pub unsafe fn new(base_address: usize) -> Self {
        Self {
            registers: &mut *(base_address as *mut Registers),
        }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &mut Registers {
        self.registers
    }

    pub fn is_enabled(&self) -> bool {
        self.registers.counter_control.is_set(Control::EN)
    }

    pub fn enable(&mut self) {
        self.registers.counter_control.modify(Control::EN::SET)
    }

    pub fn disable(&mut self) {
        self.registers.counter_control.modify(Control::EN::CLEAR)
    }

    pub fn frequency(&self) -> u32 {
        self.registers.base_frequency_id.get()
    }
}
