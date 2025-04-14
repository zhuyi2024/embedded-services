#![no_std]

/// NVRAM platform service abstraction
pub mod nvram;

// CRC service abstraction
pub mod embedded_crc;

#[cfg(any(feature = "imxrt", feature = "imxrt685"))]
pub mod imxrt;

#[cfg(any(feature = "imxrt", feature = "imxrt685"))]
pub(crate) use imxrt::*;

#[cfg(not(any(feature = "imxrt", feature = "imxrt685")))]
pub(crate) mod defaults;

#[cfg(not(any(feature = "imxrt", feature = "imxrt685")))]
pub(crate) use defaults::*;

#[cfg(test)]
mod tests {
    use super::*;
}
