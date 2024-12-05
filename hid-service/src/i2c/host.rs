//! I2C<->HID bridge
use core::borrow::Borrow;

use super::{Command as I2cCommand, I2cSlaveAsync};
use crate::Error;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, signal::Signal};
use embassy_time::{with_timeout, Duration};
use embedded_services::buffer::OwnedRef;
use embedded_services::{
    error,
    hid::{self, DeviceId},
    trace,
    transport::{self, Endpoint, EndpointLink, External, MessageDelegate},
};

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
    _buffer: OwnedRef<'static, u8>,
}

impl Host {
    pub fn new(id: DeviceId, buffer: OwnedRef<'static, u8>) -> Self {
        Host {
            id,
            tp: EndpointLink::uninit(Endpoint::External(External::Host)),
            response: Signal::new(),
            _buffer: buffer,
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
                I2cCommand::Write => return Ok(Access::Write),
                _ => unimplemented!(),
            }
        }
    }

    pub async fn process_request(&self, bus: &mut impl I2cSlaveAsync, access: Access) -> Result<(), Error> {
        match access {
            Access::Write => self.process_register_access(bus).await,
            _ => unimplemented!(),
        }
    }

    pub async fn send_response(&self, bus: &mut impl I2cSlaveAsync) -> Result<(), Error> {
        if let Some(response) = self.response.wait().await {
            match response {
                hid::Response::Descriptor(_) => trace!("Sending descriptor"),
                hid::Response::ReportDescriptor(_) => trace!("Sending report descriptor"),
                _ => trace!("Other response"),
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
                hid::Response::Descriptor(data) | hid::Response::ReportDescriptor(data) => {
                    let bytes = data.borrow();
                    self.write_bus(bus, DEVICE_RESPONSE_TIMEOUT_MS, bytes.borrow()).await
                }
                _ => unimplemented!(),
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
