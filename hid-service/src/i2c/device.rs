use core::borrow::BorrowMut;
use core::cell::Cell;
#[cfg(not(feature = "defmt"))]
use core::fmt::Debug;
use core::marker::PhantomData;

use embedded_hal_async::i2c::{AddressMode, I2c};
use embedded_services::buffer::*;
use embedded_services::hid::DeviceContainer;
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

    pub async fn process_request(&self, bus: &mut impl I2c<A, Error = E>) -> Result<(), Error> {
        let req = self.device.wait_request().await;

        let response = match req {
            hid::Request::Descriptor => {
                let desc = self.read_hid_descriptor(bus).await?;
                Some(hid::Response::Descriptor(desc))
            }
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
