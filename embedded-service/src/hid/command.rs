//! Data structures and code for handling HID commands
use core::borrow::Borrow;

use super::{Error, ReportId};
use crate::buffer::SharedRef;

/// HID report types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReportType {
    /// Input report
    Input,
    /// Output report
    Output,
    /// Feature report
    Feature,
}

const FEATURE_MASK: u16 = 0x30;
const FEATURE_SHIFT: u16 = 4;

impl TryFrom<u16> for ReportType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match (value & FEATURE_MASK) >> FEATURE_SHIFT {
            0x01 => Ok(ReportType::Input),
            0x02 => Ok(ReportType::Output),
            0x03 => Ok(ReportType::Feature),
            _ => Err(()),
        }
    }
}

impl Into<u16> for ReportType {
    fn into(self) -> u16 {
        match self {
            ReportType::Input => 0x01 << FEATURE_SHIFT,
            ReportType::Output => 0x02 << FEATURE_SHIFT,
            ReportType::Feature => 0x03 << FEATURE_SHIFT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
/// Power state
pub enum PowerState {
    /// On
    On,
    /// Sleep
    Sleep,
}

const POWER_STATE_MASK: u16 = 0x3;
impl TryFrom<u16> for PowerState {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value & POWER_STATE_MASK {
            0x0 => Ok(PowerState::On),
            0x1 => Ok(PowerState::Sleep),
            _ => Err(()),
        }
    }
}

impl Into<u16> for PowerState {
    fn into(self) -> u16 {
        match self {
            PowerState::On => 0x0,
            PowerState::Sleep => 0x1,
        }
    }
}

/// Report frequency, see spec for more details
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum ReportFreq {
    Infinite,
    Msecs(u16),
}

impl TryFrom<u16> for ReportFreq {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(ReportFreq::Infinite),
            _ => Ok(ReportFreq::Msecs(value)),
        }
    }
}

impl Into<u16> for ReportFreq {
    fn into(self) -> u16 {
        match self {
            ReportFreq::Infinite => 0x0,
            ReportFreq::Msecs(value) => value,
        }
    }
}

/// HID device protocol, see spec for more details
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Protocol {
    Boot,
    Report,
}

impl TryFrom<u16> for Protocol {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Protocol::Boot),
            0x1 => Ok(Protocol::Report),
            _ => Err(()),
        }
    }
}

impl Into<u16> for Protocol {
    fn into(self) -> u16 {
        match self {
            Protocol::Boot => 0x0,
            Protocol::Report => 0x1,
        }
    }
}

/// Command opcodes, see spec for more details
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum CommandOpcode {
    Reset,
    GetReport,
    SetReport,
    GetIdle,
    SetIdle,
    GetProtocol,
    SetProtocol,
    SetPower,
    Vendor,
}

const OPCODE_MASK: u16 = 0xf00;
const OPCODE_SHIFT: u16 = 8;

impl TryFrom<u16> for CommandOpcode {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match (value & OPCODE_MASK) >> OPCODE_SHIFT {
            0x01 => Ok(CommandOpcode::Reset),
            0x02 => Ok(CommandOpcode::GetReport),
            0x03 => Ok(CommandOpcode::SetReport),
            0x04 => Ok(CommandOpcode::GetIdle),
            0x05 => Ok(CommandOpcode::SetIdle),
            0x06 => Ok(CommandOpcode::GetProtocol),
            0x07 => Ok(CommandOpcode::SetProtocol),
            0x08 => Ok(CommandOpcode::SetPower),
            0x0e => Ok(CommandOpcode::Vendor),
            _ => Err(()),
        }
    }
}

impl Into<u16> for CommandOpcode {
    fn into(self) -> u16 {
        match self {
            CommandOpcode::Reset => 0x01 << OPCODE_SHIFT,
            CommandOpcode::GetReport => 0x02 << OPCODE_SHIFT,
            CommandOpcode::SetReport => 0x03 << OPCODE_SHIFT,
            CommandOpcode::GetIdle => 0x04 << OPCODE_SHIFT,
            CommandOpcode::SetIdle => 0x05 << OPCODE_SHIFT,
            CommandOpcode::GetProtocol => 0x06 << OPCODE_SHIFT,
            CommandOpcode::SetProtocol => 0x07 << OPCODE_SHIFT,
            CommandOpcode::SetPower => 0x08 << OPCODE_SHIFT,
            CommandOpcode::Vendor => 0x0e << OPCODE_SHIFT,
        }
    }
}

impl CommandOpcode {
    /// Return true if the command has data to read from the host
    pub fn requires_host_data(&self) -> bool {
        match self {
            CommandOpcode::SetReport | CommandOpcode::SetIdle | CommandOpcode::Vendor => true,
            _ => false,
        }
    }

    /// Return true if the command requires a report ID
    pub fn requires_report_id(&self) -> bool {
        match self {
            CommandOpcode::GetReport | CommandOpcode::SetReport | CommandOpcode::GetIdle | CommandOpcode::SetIdle => {
                true
            }
            _ => false,
        }
    }

    /// Return true if the command has a response read from the data register
    pub fn has_response(&self) -> bool {
        match self {
            CommandOpcode::GetReport | CommandOpcode::GetIdle | CommandOpcode::GetProtocol => true,
            _ => false,
        }
    }
}

/// Host to device commands, see spec for more details
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[allow(missing_docs)]
pub enum Command<'a> {
    Reset,
    GetReport(ReportType, ReportId),
    SetReport(ReportType, ReportId, SharedRef<'a, u8>),
    GetIdle(ReportId),
    SetIdle(ReportId, ReportFreq),
    GetProtocol,
    SetProtocol(Protocol),
    SetPower(PowerState),
    Vendor,
}

impl<'a> Command<'a> {
    /// Get the opcode for the command
    pub fn opcode(&self) -> CommandOpcode {
        match self {
            Command::Reset => CommandOpcode::Reset,
            Command::GetReport(_, _) => CommandOpcode::GetReport,
            Command::SetReport(_, _, _) => CommandOpcode::SetReport,
            Command::GetIdle(_) => CommandOpcode::GetIdle,
            Command::SetIdle(_, _) => CommandOpcode::SetIdle,
            Command::GetProtocol => CommandOpcode::GetProtocol,
            Command::SetProtocol(_) => CommandOpcode::SetProtocol,
            Command::SetPower(_) => CommandOpcode::SetPower,
            Command::Vendor => CommandOpcode::Vendor,
        }
    }
}

/// Device command response, GetReport uses the standard report responses
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CommandResponse {
    /// Get idle response
    GetIdle(ReportFreq),
    /// Get protocol response
    GetProtocol(Protocol),
    /// Vendor specific response
    Vendor,
}

/// Value for extended report ID
pub const EXTENDED_REPORT_ID: u8 = 0xf;
const REPORT_ID_MASK: u16 = 0xf;

impl ReportId {
    /// Get report ID from command
    pub fn from_command(cmd: u16) -> ReportId {
        ReportId((cmd & REPORT_ID_MASK) as u8)
    }

    /// Check if the command has extended report ID
    pub fn has_extended_report_id(cmd: u16) -> bool {
        Self::from_command(cmd).0 == EXTENDED_REPORT_ID
    }
}

impl<'a> Command<'a> {
    /// Creates a new command with validation
    pub fn new(
        cmd: u16,
        opcode: CommandOpcode,
        report_type: Option<ReportType>,
        report_id: Option<ReportId>,
        data: Option<SharedRef<'a, u8>>,
    ) -> Result<Self, Error> {
        if opcode.requires_report_id() && report_id.is_none() {
            return Err(Error::RequiresReportId);
        }

        if opcode.requires_host_data() && data.is_none() {
            // Vendor defined commands might or might not have data with them
            if opcode != CommandOpcode::Vendor {
                return Err(Error::RequiresData);
            }
        }

        let report_type = report_type.ok_or_else(|| Error::InvalidReportType);
        let command = match opcode {
            CommandOpcode::Reset => Command::Reset,
            CommandOpcode::GetReport => {
                if report_type? == ReportType::Input || report_type? == ReportType::Feature {
                    Command::GetReport(report_type?, report_id.unwrap())
                } else {
                    return Err(Error::InvalidReportType);
                }
            }
            CommandOpcode::SetReport => {
                if report_type? == ReportType::Output || report_type? == ReportType::Feature {
                    Command::SetReport(report_type?, report_id.unwrap(), data.unwrap())
                } else {
                    return Err(Error::InvalidReportType);
                }
            }
            CommandOpcode::GetIdle => Command::GetIdle(report_id.unwrap()),
            CommandOpcode::SetIdle => Command::SetIdle(
                report_id.unwrap(),
                cmd.try_into().map_err(|_| Error::InvalidReportFreq)?,
            ),
            CommandOpcode::GetProtocol => Command::GetProtocol,
            CommandOpcode::SetProtocol => Command::SetProtocol(cmd.try_into().map_err(|_| Error::InvalidData)?),
            CommandOpcode::SetPower => Command::SetPower(cmd.try_into().map_err(|_| Error::InvalidData)?),
            CommandOpcode::Vendor => Command::Vendor,
        };

        Ok(command)
    }

    fn encode_common(
        buf: &mut [u8],
        opcode: CommandOpcode,
        report_type: Option<ReportType>,
        report_id: ReportId,
    ) -> Result<(usize, &mut [u8]), Error> {
        let mut val: u16 = opcode.into();

        val |= report_type.map_or(0, |x| x.into());

        if report_id.0 >= EXTENDED_REPORT_ID {
            if buf.len() < 3 {
                return Err(Error::InvalidSize);
            }

            val |= EXTENDED_REPORT_ID as u16;

            buf[0..2].copy_from_slice(&val.to_le_bytes());
            buf[2] = report_id.0;

            Ok((3, &mut buf[3..]))
        } else {
            val |= report_id.0 as u16;

            buf[0..2].copy_from_slice(&val.to_le_bytes());
            Ok((2, &mut buf[2..]))
        }
    }

    fn encode_basic_op(buf: &mut [u8], opcode: CommandOpcode) -> Result<(usize, &mut [u8]), Error> {
        if buf.len() < 2 {
            return Err(Error::InvalidSize);
        }

        buf[0..2].copy_from_slice(&<CommandOpcode as Into<u16>>::into(opcode).to_le_bytes());
        Ok((2, &mut buf[2..]))
    }

    fn encode_register(buf: &mut [u8], reg: Option<u16>) -> Result<(usize, &mut [u8]), Error> {
        if let Some(reg) = reg {
            if buf.len() < 2 {
                return Err(Error::InvalidSize);
            }
            buf[0..2].copy_from_slice(&reg.to_le_bytes());
            Ok((2, &mut buf[2..]))
        } else {
            Ok((0, buf))
        }
    }

    /// Encodes a u16 value into a slice, prefixed by a length
    fn encode_value<T: Into<u16>>(buf: &mut [u8], value: T) -> Result<(usize, &mut [u8]), Error> {
        if buf.len() < 4 {
            return Err(Error::InvalidSize);
        }
        buf[0..2].copy_from_slice(&4u16.to_le_bytes());
        buf[2..4].copy_from_slice(&value.into().to_le_bytes());
        Ok((4, &mut buf[2..]))
    }

    fn encode_data<'b>(buf: &'b mut [u8], data: &[u8]) -> Result<(usize, &'b mut [u8]), Error> {
        // +2 to encode the length of the data
        let total_len = data.len() + 2;
        if buf.len() < total_len {
            return Err(Error::InvalidSize);
        }

        buf[0..2].copy_from_slice(&(total_len as u16).to_le_bytes());
        buf[2..data.len() + 2].copy_from_slice(data);
        Ok((total_len, &mut buf[data.len()..]))
    }

    /// Encode the command into a slice, returns number of bytes written
    /// If command_reg or data_reg is provided, those addresses will be encoded into the buffer as well
    pub fn encode_into_slice(
        &self,
        buf: &mut [u8],
        command_reg: Option<u16>,
        data_reg: Option<u16>,
    ) -> Result<usize, Error> {
        let mut len = 0;
        let buf: &mut [u8] = buf;

        // Encode command register address
        let (command_len, buf) = Self::encode_register(buf, command_reg)?;
        len += command_len;

        match self {
            Command::Reset => {
                let (command_len, _) = Self::encode_basic_op(buf, CommandOpcode::Reset)?;
                len += command_len;
            }
            Command::GetReport(report_type, report_id) => {
                let (command_len, buf) =
                    Self::encode_common(buf, CommandOpcode::GetReport, Some(*report_type), *report_id)?;
                len += command_len;

                // Encode data register address
                let (register_len, _) = Self::encode_register(buf, data_reg)?;
                len += register_len;
            }
            Command::SetReport(report_type, report_id, data) => {
                let borrow = data.borrow();
                let data: &[u8] = borrow.borrow();

                let (command_len, buf) =
                    Self::encode_common(buf, CommandOpcode::SetReport, Some(*report_type), *report_id)?;
                len += command_len;

                // Encode data register address
                let (register_len, buf) = Self::encode_register(buf, data_reg)?;
                len += register_len;

                // Encode report data
                let (data_len, _) = Self::encode_data(buf, data)?;
                len += data_len
            }
            Command::GetIdle(report_id) => {
                let (command_len, buf) = Self::encode_common(buf, CommandOpcode::GetIdle, None, *report_id)?;
                len += command_len;

                // Encode data register address
                let (register_len, _) = Self::encode_register(buf, data_reg)?;
                len += register_len;
            }
            Command::SetIdle(report_id, freq) => {
                let (command_len, buf) = Self::encode_common(buf, CommandOpcode::SetIdle, None, *report_id)?;
                len += command_len;

                if buf.len() < 4 {
                    return Err(Error::InvalidSize);
                }

                // Encode data register address
                let (register_len, buf) = Self::encode_register(buf, data_reg)?;
                len += register_len;

                // Include data length
                let (data_len, _) = Self::encode_value(buf, *freq)?;
                len += data_len;
            }
            Command::GetProtocol => {
                let (command_len, buf) = Self::encode_basic_op(buf, CommandOpcode::GetProtocol)?;
                len += command_len;

                // Encode data register address
                let (register_len, _) = Self::encode_register(buf, data_reg)?;
                len += register_len;
            }
            Command::SetProtocol(protocol) => {
                let (command_len, buf) = Self::encode_basic_op(buf, CommandOpcode::SetProtocol)?;
                len += command_len;

                // Encode data register address
                let (register_len, buf) = Self::encode_register(buf, data_reg)?;
                len += register_len;

                // Encode data
                let (data_len, _) = Self::encode_value(buf, *protocol)?;
                len += data_len;
            }
            Command::SetPower(state) => {
                let opcode: u16 = CommandOpcode::SetPower.into();
                let state: u16 = (*state).into();
                buf[0..2].copy_from_slice(&(opcode | state).to_le_bytes());
                len += 2;
            }
            Command::Vendor => {
                let (command_len, _) = Self::encode_basic_op(buf, CommandOpcode::Vendor)?;
                len += command_len;
            }
        }

        Ok(len)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::define_static_buffer;

    const CMD_REG: u16 = 0x0005;
    const DATA_REG: u16 = 0x0006;
    const REPORT_ID: ReportId = ReportId(8);
    const EXT_REPORT_ID: ReportId = ReportId(EXTENDED_REPORT_ID);

    #[test]
    fn test_serialize_reset() {
        let mut test_buffer = [0u8; 4];

        // Test basic functionality
        let len = Command::Reset.encode_into_slice(&mut test_buffer, None, None).unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x01]);

        // Test with command register
        test_buffer.fill(0);
        let len = Command::Reset
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x00, 0x1]);
    }

    #[test]
    fn test_serialize_get_report() {
        let mut test_buffer = [0u8; 7];

        // Test input report
        let len = Command::GetReport(ReportType::Input, REPORT_ID)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x18, 0x02]);

        // Test feature report
        test_buffer.fill(0);
        let len = Command::GetReport(ReportType::Feature, REPORT_ID)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x38, 0x02]);

        // Test extended report
        test_buffer.fill(0);
        let len = Command::GetReport(ReportType::Input, EXT_REPORT_ID)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x1f, 0x02, EXTENDED_REPORT_ID]);

        // Test standard report id with registers
        test_buffer.fill(0);
        let len = Command::GetReport(ReportType::Feature, REPORT_ID)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x38, 0x02, 0x06, 0x00]);

        // Test extended report id with registers
        test_buffer.fill(0);
        let len = Command::GetReport(ReportType::Input, EXT_REPORT_ID)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x1f, 0x02, EXTENDED_REPORT_ID, 0x06, 0x00]
        );
    }

    #[test]
    fn test_serialize_set_report() {
        let mut test_buffer = [0u8; 11];
        define_static_buffer!(data_buffer, u8, [0x00, 0x00]);

        let data = data_buffer::get_mut().unwrap();

        // Test output report
        let len = Command::SetReport(ReportType::Output, REPORT_ID, data.reference())
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x28, 0x03, 0x04, 0x00, 0x00, 0x00]);

        // Test feature report
        test_buffer.fill(0);
        let len = Command::SetReport(ReportType::Feature, REPORT_ID, data.reference())
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x38, 0x03, 0x04, 0x00, 0x00, 0x00]);

        // Test extended report
        test_buffer.fill(0);
        let len = Command::SetReport(ReportType::Output, EXT_REPORT_ID, data.reference())
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x2f, 0x03, EXTENDED_REPORT_ID, 0x04, 0x00, 0x00, 0x00]
        );

        // Test standard report id with registers
        test_buffer.fill(0);
        let len = Command::SetReport(ReportType::Output, REPORT_ID, data.reference())
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x28, 0x03, 0x06, 0x00, 0x04, 0x00, 0x00, 0x00]
        );

        // Test extended report id with registers
        test_buffer.fill(0);
        let len = Command::SetReport(ReportType::Output, EXT_REPORT_ID, data.reference())
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [
                0x05,
                0x00,
                0x2f,
                0x03,
                EXTENDED_REPORT_ID,
                0x06,
                0x00,
                0x04,
                0x00,
                0x00,
                0x00
            ]
        );
    }

    #[test]
    fn test_serialize_get_idle() {
        let mut test_buffer = [0u8; 7];

        // Test standard report id
        let len = Command::GetIdle(REPORT_ID)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x08, 0x04]);

        // Test extended report id
        test_buffer.fill(0);
        let len = Command::GetIdle(EXT_REPORT_ID)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x0f, 0x04, EXTENDED_REPORT_ID]);

        // Test standard report id with registers
        test_buffer.fill(0);
        let len = Command::GetIdle(REPORT_ID)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x08, 0x04, 0x06, 0x00]);

        // Test extended report id with registers
        test_buffer.fill(0);
        let len = Command::GetIdle(EXT_REPORT_ID)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x0f, 0x04, EXTENDED_REPORT_ID, 0x06, 0x00]
        );
    }

    #[test]
    fn test_serialize_set_idle() {
        let mut test_buffer = [0u8; 11];

        // Test standard report id
        let len = Command::SetIdle(REPORT_ID, ReportFreq::Infinite)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x08, 0x05, 0x04, 0x00, 0x00, 0x00]);

        test_buffer.fill(0);
        let len = Command::SetIdle(REPORT_ID, ReportFreq::Msecs(0x0203))
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x08, 0x05, 0x04, 0x00, 0x03, 0x02]);

        // Test extended report id
        test_buffer.fill(0);
        let len = Command::SetIdle(EXT_REPORT_ID, ReportFreq::Infinite)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x0f, 0x05, EXTENDED_REPORT_ID, 0x04, 0x00, 0x00, 0x00]
        );

        test_buffer.fill(0);
        let len = Command::SetIdle(EXT_REPORT_ID, ReportFreq::Msecs(0x0203))
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x0f, 0x05, EXTENDED_REPORT_ID, 0x04, 0x00, 0x03, 0x02]
        );

        // Test standard report id with registers
        test_buffer.fill(0);
        let len = Command::SetIdle(REPORT_ID, ReportFreq::Infinite)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x08, 0x05, 0x06, 0x00, 0x04, 0x00, 0x00, 0x00]
        );

        test_buffer.fill(0);
        let len = Command::SetIdle(REPORT_ID, ReportFreq::Msecs(0x0203))
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x08, 0x05, 0x06, 0x00, 0x04, 0x00, 0x03, 0x02]
        );

        // Test extended report id with registers
        test_buffer.fill(0);
        let len = Command::SetIdle(EXT_REPORT_ID, ReportFreq::Infinite)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [
                0x05,
                0x00,
                0x0f,
                0x05,
                EXTENDED_REPORT_ID,
                0x06,
                0x00,
                0x04,
                0x00,
                0x00,
                0x00
            ]
        );

        test_buffer.fill(0);
        let len = Command::SetIdle(EXT_REPORT_ID, ReportFreq::Msecs(0x0203))
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [
                0x05,
                0x00,
                0x0f,
                0x05,
                EXTENDED_REPORT_ID,
                0x06,
                0x00,
                0x04,
                0x00,
                0x03,
                0x02
            ]
        );
    }

    #[test]
    fn test_serialize_get_protocol() {
        let mut test_buffer = [0u8; 6];

        // Test basic functionality
        let len = Command::GetProtocol
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x06]);

        // Test with command register
        test_buffer.fill(0);
        let len = Command::GetProtocol
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x00, 0x06, 0x06, 0x00]);
    }

    #[test]
    fn test_serialized_set_protocol() {
        let mut test_buffer = [0u8; 10];

        // Test basic functionality
        let len = Command::SetProtocol(Protocol::Boot)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x07, 0x04, 0x00, 0x00, 0x00]);

        test_buffer.fill(0);
        let len = Command::SetProtocol(Protocol::Report)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x07, 0x04, 0x00, 0x01, 0x00]);

        // Test with registers
        test_buffer.fill(0);
        let len = Command::SetProtocol(Protocol::Boot)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x00, 0x07, 0x06, 0x00, 0x04, 0x00, 0x00, 0x00]
        );

        test_buffer.fill(0);
        let len = Command::SetProtocol(Protocol::Report)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), Some(DATA_REG))
            .unwrap();
        assert_eq!(
            &test_buffer[0..len],
            [0x05, 0x00, 0x00, 0x07, 0x06, 0x00, 0x04, 0x00, 0x01, 0x00]
        );
    }

    #[test]
    fn test_serialized_set_power() {
        let mut test_buffer = [0u8; 4];

        // Test basic functionality
        let len = Command::SetPower(PowerState::On)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x08]);

        test_buffer.fill(0);
        let len = Command::SetPower(PowerState::Sleep)
            .encode_into_slice(&mut test_buffer, None, None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x01, 0x08]);

        // Test with command register
        test_buffer.fill(0);
        let len = Command::SetPower(PowerState::On)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x00, 0x08]);

        test_buffer.fill(0);
        let len = Command::SetPower(PowerState::Sleep)
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x01, 0x08]);
    }

    #[test]
    fn test_serialized_vendor() {
        let mut test_buffer = [0xffu8; 4];

        // Test basic functionality
        test_buffer.fill(0);
        let len = Command::Vendor.encode_into_slice(&mut test_buffer, None, None).unwrap();
        assert_eq!(&test_buffer[0..len], [0x00, 0x0e]);

        // Test with command register
        test_buffer.fill(0);
        let len = Command::Vendor
            .encode_into_slice(&mut test_buffer, Some(CMD_REG), None)
            .unwrap();
        assert_eq!(&test_buffer[0..len], [0x05, 0x00, 0x00, 0x0e]);
    }
}
