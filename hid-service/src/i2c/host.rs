//! I2C<->HID bridge
use core::borrow::{Borrow, BorrowMut};

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{with_timeout, Duration};
use embedded_services::buffer::OwnedRef;
use embedded_services::hid::{self, CommandOpcode, DeviceId};
use embedded_services::transport::{self, Endpoint, EndpointLink, External, MessageDelegate};
use embedded_services::{error, trace};

use super::{Command as I2cCommand, I2cSlaveAsync};
use crate::Error;

const DEVICE_RESPONSE_TIMEOUT_MS: u64 = 200;
const DATA_READ_TIMEOUT_MS: u64 = 50;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
}

pub struct Host {
    id: DeviceId,
    pub tp: EndpointLink,
    response: Signal<NoopRawMutex, Option<hid::Response<'static>>>,
    buffer: OwnedRef<'static, u8>,
}

impl Host {
    pub fn new(id: DeviceId, buffer: OwnedRef<'static, u8>) -> Self {
        Host {
            id,
            tp: EndpointLink::uninit(Endpoint::External(External::Host)),
            response: Signal::new(),
            buffer,
        }
    }

    async fn read_bus(&self, bus: &mut impl I2cSlaveAsync, timeout_ms: u64, buffer: &mut [u8]) -> Result<(), Error> {
        let result = with_timeout(Duration::from_millis(timeout_ms), bus.respond_to_write(buffer)).await;
        if result.is_err() {
            error!("Response timeout");
            return Err(Error::Timeout);
        }

        if let Err(e) = result.unwrap() {
            error!("Bus error {:?}", e);
            return Err(Error::Bus);
        }

        Ok(())
    }

    async fn write_bus(&self, bus: &mut impl I2cSlaveAsync, timeout_ms: u64, buffer: &[u8]) -> Result<(), Error> {
        // Send response, timeout if the host doesn't read so we don't get stuck here
        trace!("Sending {} bytes", buffer.len());
        let result = with_timeout(Duration::from_millis(timeout_ms), bus.respond_to_read(buffer)).await;
        if result.is_err() {
            error!("Response timeout");
            return Err(Error::Timeout);
        }

        if let Err(e) = result.unwrap() {
            error!("Bus error {:?}", e);
            return Err(Error::Bus);
        }

        trace!("Response sent");
        Ok(())
    }

    async fn process_command(
        &self,
        bus: &mut impl I2cSlaveAsync,
        device: &hid::Device,
    ) -> Result<hid::Command<'static>, Error> {
        trace!("Waiting for command");
        let mut cmd = [0u8; 2];
        self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut cmd).await?;

        let cmd = u16::from_le_bytes(cmd);
        let opcode = CommandOpcode::try_from(cmd);
        if opcode.is_err() {
            error!("Invalid command {:#x}", cmd);
            return Err(Error::InvalidCommand);
        }

        trace!("Command {:#x}", cmd);
        // Get report ID
        let opcode = opcode.unwrap();
        trace!("Opcode {:?}", opcode);
        let report_id = if opcode.requires_report_id() {
            // See if we need to read another byte for the full report ID
            if hid::ReportId::has_extended_report_id(cmd) {
                trace!("Reading extended report ID");
                let mut report_id = [0u8; 1];
                self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut report_id).await?;

                Some(hid::ReportId(report_id[0]))
            } else {
                Some(hid::ReportId::from_command(cmd))
            }
        } else {
            None
        };

        // Read data from host through data register
        let buffer = if opcode.requires_host_data() || opcode.has_response() {
            let mut addr = [0u8; 2];
            // If the command has a response then we only needed to consume the data register address
            trace!("Waiting for host data access");
            self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut addr).await?;

            let reg = u16::from_le_bytes(addr);
            if reg != device.regs.data_reg {
                error!("Invalid data register {:#x}", reg);
                return Err(Error::InvalidAddress);
            }

            if opcode.requires_host_data() {
                trace!("Waiting for data");
                let mut borrow = self.buffer.borrow_mut();
                let buffer: &mut [u8] = borrow.borrow_mut();

                self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut buffer[0..2]).await?;

                let length = u16::from_le_bytes([buffer[0], buffer[1]]);
                if buffer.len() < length as usize {
                    error!("Buffer overrun: {}", length);
                    return Err(Error::InvalidSize);
                }

                trace!("Reading {} bytes", length);
                self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut buffer[2..length as usize])
                    .await?;
                Some(self.buffer.reference().slice(2..length as usize))
            } else {
                None
            }
        } else {
            None
        };

        // Create command
        let report_type = hid::ReportType::try_from(cmd).ok();
        let command = hid::Command::new(cmd, opcode, report_type, report_id, buffer);
        if let Err(e) = command {
            error!("Invalid command {:?}", e);
            return Err(Error::InvalidCommand);
        }

        Ok(command.unwrap())
    }

    /// Handle an access to a specific register
    async fn process_register_access(&self, bus: &mut impl I2cSlaveAsync) -> Result<(), Error> {
        let mut reg = [0u8; 2];
        trace!("Waiting for register address");
        self.read_bus(bus, DATA_READ_TIMEOUT_MS, &mut reg).await?;

        let reg = u16::from_le_bytes(reg);
        trace!("Register address {:#x}", reg);
        if let Some(device) = hid::get_device(self.id).await {
            let request = if reg == device.regs.hid_desc_reg {
                hid::Request::Descriptor
            } else if reg == device.regs.report_desc_reg {
                hid::Request::ReportDescriptor
            } else if reg == device.regs.input_reg {
                hid::Request::InputReport
            } else if reg == device.regs.command_reg {
                hid::Request::Command(self.process_command(bus, device).await?)
            } else {
                error!("Unexpected request address {:#x}", reg);
                return Err(Error::InvalidAddress);
            };

            hid::send_request(&self.tp, self.id, request)
                .await
                .map_err(|_| Error::Transport)?;

            trace!("Request processed");
            Ok(())
        } else {
            error!("Invalid device id {}", self.id.0);
            Err(Error::InvalidDevice)
        }
    }

    async fn process_read(&self) -> Result<(), Error> {
        trace!("Got input report read request");
        hid::send_request(&self.tp, self.id, hid::Request::InputReport)
            .await
            .map_err(|_| Error::Transport)
    }

    /// Process a request from the host
    pub async fn wait_request(&self, bus: &mut impl I2cSlaveAsync) -> Result<Access, Error> {
        // Wait for HID register address
        loop {
            trace!("Waiting for host");

            let result = bus.listen().await;
            if let Err(e) = result {
                error!("Bus error {:?}", e);
                return Err(Error::Bus);
            }

            match result.unwrap() {
                I2cCommand::Probe => continue,
                I2cCommand::Read => return Ok(Access::Read),
                I2cCommand::Write => return Ok(Access::Write),
            }
        }
    }

    pub async fn process_request(&self, bus: &mut impl I2cSlaveAsync, access: Access) -> Result<(), Error> {
        match access {
            Access::Read => self.process_read().await,
            Access::Write => self.process_register_access(bus).await,
        }
    }

    pub async fn send_response(&self, bus: &mut impl I2cSlaveAsync) -> Result<(), Error> {
        if let Some(response) = self.response.wait().await {
            match response {
                hid::Response::Descriptor(_) => trace!("Sending descriptor"),
                hid::Response::ReportDescriptor(_) => trace!("Sending report descriptor"),
                hid::Response::InputReport(_) => trace!("Sending input report"),
                hid::Response::FeatureReport(_) => trace!("Sending feature report"),
                hid::Response::Command(_) => trace!("Sending command"),
            }

            // Wait for the read from the host
            // Input reports just a read so we don't need to wait for one
            if !matches!(response, hid::Response::InputReport(_)) {
                let result = bus.listen().await;
                if let Err(e) = result {
                    error!("Bus error {:?}", e);
                    return Err(Error::Bus);
                }

                if !matches!(result.unwrap(), I2cCommand::Read) {
                    error!("Expected read");
                    return Err(Error::Bus);
                }
            }

            let result = match response {
                hid::Response::Descriptor(data)
                | hid::Response::ReportDescriptor(data)
                | hid::Response::InputReport(data)
                | hid::Response::FeatureReport(data) => {
                    let bytes = data.borrow();
                    self.write_bus(bus, DEVICE_RESPONSE_TIMEOUT_MS, bytes.borrow()).await
                }
                hid::Response::Command(cmd) => match cmd {
                    hid::CommandResponse::GetIdle(freq) => {
                        let freq: u16 = freq.into();
                        let mut buffer = [0u8; 2];
                        buffer.copy_from_slice(freq.to_le_bytes().as_slice());
                        self.write_bus(bus, DEVICE_RESPONSE_TIMEOUT_MS, &buffer).await
                    }
                    hid::CommandResponse::GetProtocol(protocol) => {
                        let protocol: u16 = protocol.into();
                        let mut buffer = [0u8; 2];
                        buffer.copy_from_slice(protocol.to_le_bytes().as_slice());
                        self.write_bus(bus, DEVICE_RESPONSE_TIMEOUT_MS, &buffer).await
                    }
                    hid::CommandResponse::Vendor => Ok(()),
                },
            };

            result
        } else {
            Ok(())
        }
    }

    pub async fn process(&self, bus: &mut impl I2cSlaveAsync) -> Result<(), Error> {
        let access = self.wait_request(bus).await?;
        self.process_request(bus, access).await?;
        self.send_response(bus).await
    }
}

impl MessageDelegate for Host {
    fn process(&self, message: &transport::Message) {
        if message.to != Endpoint::External(External::Host) {
            return;
        }

        if let Some(message) = message.data.get::<hid::Message>() {
            if message.id != self.id {
                return;
            }

            if let hid::MessageData::Response(ref response) = message.data {
                self.response.signal(response.clone());
            }
        }
    }
}
