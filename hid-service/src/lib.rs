#![no_std]

pub mod i2c;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    Bus,
    Transport,
    Timeout,
    Deserialize,
    InvalidSize,
    InvalidAddress,
    InvalidDevice,
    InvalidCommand,
}
