pub(crate) mod nvram;
pub(crate) use nvram::*;

// Set embedded_crc access level outside the crate to initialize CRC hardware object
pub mod embedded_crc;
pub(crate) use embedded_crc::*;
