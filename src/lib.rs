//! A platform agnostic driver to interface with the MCP3008 / MCP3004 ADC's.
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.1
//!

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate embedded_hal as hal;

use hal::blocking::spi::Transfer;
use hal::spi::{Mode, Phase, Polarity};
use hal::digital::v2::OutputPin;

/// SPI mode
pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnFirstTransition,
    polarity: Polarity::IdleLow,
};

/// MCP3008 driver
pub struct Mcp3008<SPI, CS> {
    spi: SPI,
    cs: CS,
}

/// MCP3004 driver
pub struct Mcp3004<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS, E> Mcp3008<SPI, CS>
    where SPI: Transfer<u8, Error = E>,
          CS: OutputPin
{
    /// Creates a new driver from an SPI peripheral and a chip select
    /// digital I/O pin.
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let mcp3008 = Mcp3008 { spi: spi, cs: cs };

        Ok(mcp3008)
    }

    /// Read a MCP3008 ADC channel and return the 10 bit value as a u16
    pub fn read_channel(&mut self, ch: Channels8) -> Result<u16, E> {
        let _ = self.cs.set_low();

        let mut buffer = [0u8; 3];
        buffer[0] = 1;
        buffer[1] = ((1 << 3) | (ch as u8)) << 4;

        self.spi.transfer(&mut buffer)?;

        let _ = self.cs.set_high();

        let r = (((buffer[1] as u16) << 8) | (buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

impl<SPI, CS, E> Mcp3004<SPI, CS>
    where SPI: Transfer<u8, Error = E>,
          CS: OutputPin
{
    /// Creates a new driver from an SPI peripheral and a chip select 
    /// digital I/O pin.
    pub fn new(spi: SPI, cs: CS) -> Result<Self, E> {
        let mcp3004 = Mcp3004 { spi: spi, cs: cs };

        Ok(mcp3004)
    }

    /// Read a MCP3004 ADC channel and return the 10 bit value as a u16
    pub fn read_channel(&mut self, ch: Channels4) -> Result<u16, E> {
        let _ = self.cs.set_low();

        let mut buffer = [0u8; 3];
        buffer[0] = 1;
        buffer[1] = ((1 << 3) | (ch as u8)) << 4;

        self.spi.transfer(&mut buffer)?;

        let _ = self.cs.set_high();

        let r = (((buffer[1] as u16) << 8) | (buffer[2] as u16)) & 0x3ff;
        Ok(r)
    }
}

/// Channel list for MCP3008
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Channels8 {
    CH0,
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
}

/// Channel list for MCP3004
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub enum Channels4 {
    CH0,
    CH1,
    CH2,
    CH3,
}
