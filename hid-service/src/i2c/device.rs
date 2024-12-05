use core::borrow::BorrowMut;
use core::cell::Cell;
#[cfg(not(feature = "defmt"))]
use core::fmt::Debug;
use core::marker::PhantomData;

use embedded_hal_async::i2c::{AddressMode, I2c};
use embedded_services::buffer::*;
use embedded_services::hid::{DeviceContainer, Response};
use embedded_services::{error, hid, info, trace};

use crate::Error;

pub struct Device<
    A: AddressMode + Copy,
    #[cfg(feature = "defmt")] E: embedded_hal_async::i2c::Error + defmt::Format,
    #[cfg(not(feature = "defmt"))] E: embedded_hal_async::i2c::Error + Debug,
> {
    device: hid::Device,
    buffer: OwnedRef<'static, u8>,
    address: A,
    descriptor: Cell<Option<hid::Descriptor>>,
    _phantom: PhantomData<E>,
}

impl<
        A: AddressMode + Copy,
        #[cfg(feature = "defmt")] E: embedded_hal_async::i2c::Error + defmt::Format,
        #[cfg(not(feature = "defmt"))] E: embedded_hal_async::i2c::Error + Debug,
    > Device<A, E>
{
    pub fn new(id: hid::DeviceId, address: A, regs: hid::RegisterFile, buffer: OwnedRef<'static, u8>) -> Self {
        Self {
            device: hid::Device::new(id, regs),
            buffer,
            address,
            descriptor: Cell::new(None),
            _phantom: PhantomData,
        }
    }

    async fn get_hid_descriptor(&self, bus: &mut impl I2c<A, Error = E>) -> Result<hid::Descriptor, Error> {
        if self.descriptor.get().is_some() {
            return Ok(self.descriptor.get().unwrap());
        }

        let mut borrow = self.buffer.borrow_mut();
        let mut reg = [0u8; 2];
        let buf: &mut [u8] = borrow.borrow_mut();
        let buf = &mut buf[0..hid::DESCRIPTOR_LEN];

        reg.copy_from_slice(&self.device.regs.hid_desc_reg.to_le_bytes());
        if let Err(e) = bus.write_read(self.address, &reg, buf).await {
            error!("Failed to read HID descriptor: {:#?}", e);
            return Err(Error::Bus);
        }

        let desc = hid::Descriptor::decode_from_slice(buf).map_err(|_| Error::Deserialize)?;
        info!("HID descriptor: {:#?}", desc);
        self.descriptor.set(Some(desc));

        Ok(desc)
    }

    pub async fn read_hid_descriptor(&self, bus: &mut impl I2c<A, Error = E>) -> Result<SharedRef<'static, u8>, Error> {
        let desc = self.get_hid_descriptor(bus).await?;

        let mut borrow = self.buffer.borrow_mut();
        let buf: &mut [u8] = borrow.borrow_mut();

        let len = desc.encode_into_slice(buf).map_err(|_| Error::Deserialize)?;
        trace!("HID descriptor length: {}", len);
        Ok(self.buffer.reference().slice(0..len))
    }

    pub async fn read_report_descriptor(
        &self,
        bus: &mut impl I2c<A, Error = E>,
    ) -> Result<SharedRef<'static, u8>, Error> {
        info!("Sending report descriptor");

        let mut borrow = self.buffer.borrow_mut();
        let buf: &mut [u8] = borrow.borrow_mut();
        let desc = self.get_hid_descriptor(bus).await?;
        let reg = desc.w_report_desc_register.to_le_bytes();
        let len = desc.w_report_desc_length as usize;

        if let Err(e) = bus.write_read(self.address, &reg, &mut buf[0..len]).await {
            error!("Failed to read report descriptor: {:#?}", e);
            return Err(Error::Bus);
        }

        Ok(self.buffer.reference().slice(0..len))
    }

    pub async fn handle_input_report(&self, bus: &mut impl I2c<A, Error = E>) -> Result<SharedRef<'static, u8>, Error> {
        info!("Handling input report");
        let desc = self.get_hid_descriptor(bus).await?;

        let mut borrow = self.buffer.borrow_mut();
        let buf: &mut [u8] = borrow.borrow_mut();
        let buf = &mut buf[0..desc.w_max_input_length as usize];

        if let Err(e) = bus.read(self.address, buf).await {
            error!("Failed to read input report: {:#?}", e);
            return Err(Error::Bus);
        }

        Ok(self.buffer.reference().slice(0..desc.w_max_input_length as usize))
    }

    pub async fn handle_command(
        &self,
        bus: &mut impl I2c<A, Error = E>,
        cmd: &hid::Command<'static>,
    ) -> Result<Option<Response<'static>>, Error> {
        info!("Handling command");

        let mut borrow = self.buffer.borrow_mut();
        let buf: &mut [u8] = borrow.borrow_mut();

        let len = cmd
            .encode_into_slice(
                buf,
                Some(self.device.regs.command_reg),
                if cmd.opcode().has_response() || cmd.opcode().requires_host_data() {
                    Some(self.device.regs.data_reg)
                } else {
                    None
                },
            )
            .map_err(|_| Error::Deserialize)?;

        bus.write(self.address, &buf[..len]).await.map_err(|_| Error::Bus)?;

        if cmd.opcode().has_response() {
            trace!("Reading response");
            bus.read(self.address, buf).await.map_err(|_| Error::Bus)?;

            return Ok(Some(Response::FeatureReport(self.buffer.reference())));
        }

        Ok(None)
    }

    pub async fn process_request(&self, bus: &mut impl I2c<A, Error = E>) -> Result<(), Error> {
        let req = self.device.wait_request().await;

        let response = match req {
            hid::Request::Descriptor => {
                let desc = self.read_hid_descriptor(bus).await?;
                Some(hid::Response::Descriptor(desc))
            }
            hid::Request::ReportDescriptor => {
                let desc = self.read_report_descriptor(bus).await?;
                Some(hid::Response::ReportDescriptor(desc))
            }
            hid::Request::InputReport => {
                let report = self.handle_input_report(bus).await?;
                Some(hid::Response::InputReport(report))
            }
            hid::Request::Command(cmd) => self.handle_command(bus, &cmd).await?,
            _ => unimplemented!(),
        };

        self.device
            .send_response(response)
            .await
            .map_err(|_| Error::Transport)?;

        Ok(())
    }
}

impl<
        A: AddressMode + Copy,
        #[cfg(feature = "defmt")] E: embedded_hal_async::i2c::Error + defmt::Format,
        #[cfg(not(feature = "defmt"))] E: embedded_hal_async::i2c::Error + Debug,
    > DeviceContainer for Device<A, E>
{
    fn get_hid_device(&self) -> &hid::Device {
        &self.device
    }
}
