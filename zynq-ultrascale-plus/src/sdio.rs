use alloc::vec::Vec;
use core::fmt::{self, Debug};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};
use zynq_ultrascale_plus_modules::sdio::*;

pub struct Controller {
    registers: &'static Registers,
}

impl Debug for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Controller").finish()
    }
}

fn new_adma2_descriptor_table(mut dest_addr: u32, mut len: usize) -> Vec<u64> {
    let mut desc_table = Vec::new();

    while len > 0 {
        let entry_len = if len > 0x10000 { 0x10000 } else { len };
        let end_attr = if len == entry_len { 2 } else { 0 };
        let attrs = end_attr | 0x21;

        desc_table.push((dest_addr as u64) << 32 | (entry_len as u16 as u64) << 16 | attrs);

        len -= entry_len;
        dest_addr += entry_len as u32;
    }

    desc_table
}

impl Controller {
    /// Initiatizes and returns the SD 0 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn sd0() -> Self {
        Self::new(&mut *SD0)
    }

    /// Initiatizes and returns the SD 1 controller.
    ///
    /// # Safety
    /// Things will break spectacularly if this is called on an unsupported device or if you create
    /// multiple controllers at once.
    pub unsafe fn sd1() -> Self {
        Self::new(&mut *SD1)
    }

    /// Creates a new SD controller.
    pub fn new(registers: &'static Registers) -> Self {
        Self { registers }
    }

    /// Provides raw access to the registers.
    ///
    /// # Safety
    /// Refer to the module's reference material to understand what is and isn't safe.
    pub unsafe fn registers(&mut self) -> &Registers {
        self.registers
    }

    /// Resets and configures the controller for use.
    pub fn configure(&mut self) {
        self.registers
            .reg_softwarereset
            .write(RegSoftwarereset::SWRESET_FOR_ALL::SET);

        while self
            .registers
            .reg_softwarereset
            .is_set(RegSoftwarereset::SWRESET_FOR_ALL)
        {}

        self.registers.reg_powercontrol.write(
            RegPowercontrol::PWRCTRL_SDBUSVOLTAGE::SET + RegPowercontrol::PWRCTRL_SDBUSPOWER::SET,
        );

        unsafe { self.change_clock_frequency(0x80) };

        self.registers
            .reg_hostcontrol1
            .modify(RegHostcontrol1::HOSTCTRL1_DMASELECT.val(2));

        self.registers.reg_normalintrstsena.set(0xfeff);
        self.registers.reg_errorintrstsena.set(0x3ff);
        self.registers.reg_normalintrsigena.set(0);
        self.registers.reg_errorintrsigena.set(0);

        self.registers.reg_transfermode.write(
            RegTransfermode::XFERMODE_DMAENABLE::SET
                + RegTransfermode::XFERMODE_BLKCNTENA::SET
                + RegTransfermode::XFERMODE_DATAXFERDIR::SET,
        );

        self.registers.reg_blocksize.set(0x200);
    }

    unsafe fn change_clock_frequency(&mut self, frequency_select: u16) {
        self.registers.reg_clockcontrol.set(
            (RegClockcontrolW::CLKCTRL_SDCLKENA::CLEAR
                + RegClockcontrolW::CLKCTRL_INTCLKENA::CLEAR)
                .modify(self.registers.reg_clockcontrol.get()),
        );

        for _ in 0..1000 {}

        self.registers.reg_clockcontrol.set(
            (RegClockcontrolW::CLKCTRL_SDCLKFREQSEL.val(frequency_select)
                + RegClockcontrolW::CLKCTRL_INTCLKENA::SET)
                .modify(self.registers.reg_clockcontrol.get()),
        );

        for _ in 0..1000 {}

        while !self
            .registers
            .reg_clockcontrol
            .is_set(RegClockcontrolR::SDHCCLKGEN_INTCLKSTABLE_DSYNC)
        {}

        self.registers.reg_clockcontrol.set(
            RegClockcontrolW::CLKCTRL_SDCLKENA::SET.modify(self.registers.reg_clockcontrol.get()),
        );
    }

    /// Returns true if the card is present.
    pub fn is_present(&self) -> bool {
        self.registers
            .reg_presentstate
            .is_set(RegPresentstate::SDIF_CD_N_DSYNC)
    }

    /// Sends a command to the card.
    pub unsafe fn cmd_transfer(
        &mut self,
        cmd: u16,
        argument: u32,
        block_count: u16,
        response_type: CmdResponseType,
    ) -> Result<(), CmdTransferError> {
        if self
            .registers
            .reg_presentstate
            .is_set(RegPresentstate::PRESENTSTATE_INHIBITCMD)
        {
            return Err(CmdTransferError::CmdInProgress);
        }

        self.registers.reg_blockcount.set(block_count);
        self.registers.reg_timeoutcontrol.set(0x0e);
        self.registers.reg_argument1lo.set(argument as _);
        self.registers.reg_argument1hi.set((argument >> 16) as _);
        self.registers.reg_normalintrsts.set(0xffff);
        self.registers.reg_errorintrsts.set(0xf3ff);

        if self
            .registers
            .reg_presentstate
            .is_set(RegPresentstate::PRESENTSTATE_INHIBITDAT)
        {
            return Err(CmdTransferError::CmdInProgress);
        }

        self.registers.reg_command.write(
            RegCommand::COMMAND_CMDINDEX.val(cmd)
                + RegCommand::COMMAND_DATAPRESENT.val(if block_count > 0 { 1 } else { 0 })
                + RegCommand::COMMAND_RESPONSETYPE.val(response_type as _),
        );

        while !self
            .registers
            .reg_normalintrsts
            .is_set(RegNormalintrstsR::NORMALINTRSTS_CMDCOMPLETE)
        {}

        self.registers.reg_errorintrsts.set(0xf3ff);
        self.registers
            .reg_normalintrsts
            .write(RegNormalintrstsW::NORMALINTRSTS_CMDCOMPLETE::SET);

        Ok(())
    }

    /// Sends an application-specific command to the card.
    pub unsafe fn acmd_transfer(
        &mut self,
        cmd: u16,
        argument: u32,
        response_type: CmdResponseType,
    ) -> Result<(), CmdTransferError> {
        self.cmd_transfer(55, 0, 0, CmdResponseType::None)?;
        self.cmd_transfer(cmd, argument, 0, response_type)
    }

    pub fn initialize(mut self) -> Result<Card, InitializeError> {
        if !self.is_present() {
            return Err(InitializeError::NotPresent(self));
        }

        unsafe {
            // Reset the card.
            if let Err(e) = self.cmd_transfer(0, 0, 0, CmdResponseType::None) {
                return Err(InitializeError::CmdTransferError(e, self));
            }

            // Tell the card which voltages we support.
            if let Err(e) = self.cmd_transfer(8, 0x1aa, 0, CmdResponseType::Length48) {
                return Err(InitializeError::CmdTransferError(e, self));
            } else if self.registers.reg_response0.get() != 0x1aa {
                return Err(InitializeError::UnsupportedDevice(self));
            }

            loop {
                // Tell the card we support high-speed mode.
                if let Err(e) = self.acmd_transfer(41, 0x40300000, CmdResponseType::Length48) {
                    return Err(InitializeError::CmdTransferError(e, self));
                }

                let resp_hi = self.registers.reg_response1.get();
                if resp_hi & 0x8000 != 0 {
                    break;
                }
            }

            let id = {
                if let Err(e) = self.cmd_transfer(2, 0, 0, CmdResponseType::Length136) {
                    return Err(InitializeError::CmdTransferError(e, self));
                }

                let resp7 = self.registers.reg_response7.get();
                if resp7 & 0xff00 != 0 {
                    return Err(InitializeError::UnsupportedDevice(self));
                }

                let resp0 = self.registers.reg_response0.get();
                let resp1 = self.registers.reg_response1.get();
                let resp2 = self.registers.reg_response2.get();
                let resp3 = self.registers.reg_response3.get();
                let resp4 = self.registers.reg_response4.get();
                let resp5 = self.registers.reg_response5.get();
                let resp6 = self.registers.reg_response6.get();

                [
                    resp7 as u8,
                    (resp6 >> 8) as u8,
                    resp6 as u8,
                    (resp5 >> 8) as u8,
                    resp5 as u8,
                    (resp4 >> 8) as u8,
                    resp4 as u8,
                    (resp3 >> 8) as u8,
                    resp3 as u8,
                    (resp2 >> 8) as u8,
                    resp2 as u8,
                    (resp1 >> 8) as u8,
                    resp1 as u8,
                    (resp0 >> 8) as u8,
                    resp0 as u8,
                    0,
                ]
            };

            let relative_address = {
                if let Err(e) = self.cmd_transfer(3, 0, 0, CmdResponseType::Length48) {
                    return Err(InitializeError::CmdTransferError(e, self));
                }

                self.registers.reg_response1.get()
            };

            let _csd = {
                if let Err(e) = self.cmd_transfer(
                    9,
                    (relative_address as u32) << 16,
                    0,
                    CmdResponseType::Length136,
                ) {
                    return Err(InitializeError::CmdTransferError(e, self));
                }

                let resp7 = self.registers.reg_response7.get();
                if resp7 & 0xff00 != 0 {
                    return Err(InitializeError::UnsupportedDevice(self));
                }

                let resp0 = self.registers.reg_response0.get();
                let resp1 = self.registers.reg_response1.get();
                let resp2 = self.registers.reg_response2.get();
                let resp3 = self.registers.reg_response3.get();
                let resp4 = self.registers.reg_response4.get();
                let resp5 = self.registers.reg_response5.get();
                let resp6 = self.registers.reg_response6.get();

                [
                    resp7 as u8,
                    (resp6 >> 8) as u8,
                    resp6 as u8,
                    (resp5 >> 8) as u8,
                    resp5 as u8,
                    (resp4 >> 8) as u8,
                    resp4 as u8,
                    (resp3 >> 8) as u8,
                    resp3 as u8,
                    (resp2 >> 8) as u8,
                    resp2 as u8,
                    (resp1 >> 8) as u8,
                    resp1 as u8,
                    (resp0 >> 8) as u8,
                    resp0 as u8,
                    0,
                ]
            };

            // TODO: parse the csd, configure optimal clock speed, etc.

            // Select the card.
            if let Err(e) = self.cmd_transfer(
                7,
                (relative_address as u32) << 16,
                0,
                CmdResponseType::Length48,
            ) {
                return Err(InitializeError::CmdTransferError(e, self));
            }

            // Set the block size to 512 bytes.
            {
                if let Err(e) = self.cmd_transfer(16, 512, 0, CmdResponseType::None) {
                    return Err(InitializeError::CmdTransferError(e, self));
                }
                self.registers.reg_blocksize.set(
                    RegBlocksize::XFER_BLOCKSIZE
                        .val(512)
                        .modify(self.registers.reg_blocksize.get()),
                );
            }

            Ok(Card {
                id,
                controller: self,
            })
        }
    }
}

pub enum CmdResponseType {
    None = 0,
    Length136 = 1,
    Length48 = 2,
    Length48Busy = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CmdTransferError {
    CmdInProgress,
}

#[derive(Debug)]
pub enum InitializeError {
    NotPresent(Controller),
    CmdTransferError(CmdTransferError, Controller),
    CmdError(Controller),
    UnsupportedDevice(Controller),
}

impl From<InitializeError> for Controller {
    fn from(e: InitializeError) -> Self {
        match e {
            InitializeError::NotPresent(c) => c,
            InitializeError::CmdTransferError(_, c) => c,
            InitializeError::CmdError(c) => c,
            InitializeError::UnsupportedDevice(c) => c,
        }
    }
}

pub struct Card {
    id: [u8; 16],
    controller: Controller,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IoError {
    NotPresent,
    BadArgument,
    MemoryOutOfRange,
    UnknownError,
    CmdTransferError(CmdTransferError),
}

impl Card {
    pub fn id(&self) -> &[u8; 16] {
        &self.id
    }

    pub fn block_size(&self) -> usize {
        512
    }

    pub fn read_blocks(&mut self, block_addr: u32, dest: &mut [u8]) -> Result<(), IoError> {
        if dest.len() % self.block_size() != 0 || dest.len() == 0 {
            return Err(IoError::BadArgument);
        } else if !self.controller.is_present() {
            return Err(IoError::NotPresent);
        } else if dest.as_ptr() as u64 > 0xffffffff {
            return Err(IoError::MemoryOutOfRange);
        }

        let desc_table = new_adma2_descriptor_table(dest.as_ptr() as u32, dest.len());
        let desc_table_addr = desc_table.as_ptr() as u64;
        if desc_table_addr > 0xffffffff {
            return Err(IoError::MemoryOutOfRange);
        }

        self.controller
            .registers
            .reg_admasysaddr0
            .set(desc_table_addr as _);
        self.controller
            .registers
            .reg_admasysaddr1
            .set((desc_table_addr >> 16) as _);

        self.controller.registers.reg_transfermode.set(0x37);

        let data_addr = block_addr * self.block_size() as u32;

        unsafe {
            if let Err(e) = self.controller.cmd_transfer(
                18,
                data_addr,
                (dest.len() / self.block_size()) as _,
                CmdResponseType::Length48,
            ) {
                return Err(IoError::CmdTransferError(e));
            }
        }

        if self
            .controller
            .registers
            .reg_normalintrsts
            .is_set(RegNormalintrstsR::REG_ERRORINTRSTS)
        {
            return Err(IoError::UnknownError);
        }

        while !self
            .controller
            .registers
            .reg_normalintrsts
            .is_set(RegNormalintrstsR::NORMALINTRSTS_XFERCOMPLETE)
        {}

        self.controller
            .registers
            .reg_normalintrsts
            .write(RegNormalintrstsW::NORMALINTRSTS_XFERCOMPLETE::SET);

        Ok(())
    }

    pub fn write_blocks(&mut self, block_addr: u32, source: &[u8]) -> Result<(), IoError> {
        if source.len() % self.block_size() != 0 || source.len() == 0 {
            return Err(IoError::BadArgument);
        } else if !self.controller.is_present() {
            return Err(IoError::NotPresent);
        } else if source.as_ptr() as u64 > 0xffffffff {
            return Err(IoError::MemoryOutOfRange);
        }

        let desc_table = new_adma2_descriptor_table(source.as_ptr() as u32, source.len());
        let desc_table_addr = desc_table.as_ptr() as u64;
        if desc_table_addr > 0xffffffff {
            return Err(IoError::MemoryOutOfRange);
        }

        self.controller
            .registers
            .reg_admasysaddr0
            .set(desc_table_addr as _);
        self.controller
            .registers
            .reg_admasysaddr1
            .set((desc_table_addr >> 16) as _);

        self.controller.registers.reg_transfermode.set(0x27);

        let data_addr = block_addr * self.block_size() as u32;

        unsafe {
            if let Err(e) = self.controller.cmd_transfer(
                25,
                data_addr,
                (source.len() / self.block_size()) as _,
                CmdResponseType::Length48,
            ) {
                return Err(IoError::CmdTransferError(e));
            }
        }

        if self
            .controller
            .registers
            .reg_normalintrsts
            .is_set(RegNormalintrstsR::REG_ERRORINTRSTS)
        {
            return Err(IoError::UnknownError);
        }

        while !self
            .controller
            .registers
            .reg_normalintrsts
            .is_set(RegNormalintrstsR::NORMALINTRSTS_XFERCOMPLETE)
        {}

        self.controller
            .registers
            .reg_normalintrsts
            .write(RegNormalintrstsW::NORMALINTRSTS_XFERCOMPLETE::SET);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test(qemu_only)]
    fn test_controller() {
        let mut sd0 = unsafe { Controller::sd0() };
        let sd1 = unsafe { Controller::sd1() };

        sd0.configure();

        assert!(sd0.is_present());
        assert!(!sd1.is_present());

        let mut card = sd0.initialize().unwrap();
        assert_eq!(
            card.id(),
            &[
                0xaa, 0x58, 0x59, 0x51, 0x45, 0x4d, 0x55, 0x21, 0x01, 0xde, 0xad, 0xbe, 0xef, 0x00,
                0x62, 0x00,
            ]
        );

        let mut buffer = vec![7u8; 2 * 512];

        card.write_blocks(0, &buffer).unwrap();

        card.read_blocks(10, &mut buffer).unwrap();
        assert_eq!(&buffer[..], &[0u8; 2 * 512]);

        card.read_blocks(0, &mut buffer).unwrap();
        assert_eq!(&buffer[..], &[7u8; 2 * 512]);
    }
}
