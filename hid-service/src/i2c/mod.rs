mod device;
mod host;

pub use device::*;
pub use host::*;

pub enum Command {
    Probe,
    Write,
    Read,
}

// TODO: remove and use embedded hal trait when imxrt I2C implements it
#[allow(async_fn_in_trait)]
pub trait I2cSlaveAsync {
    async fn listen(&mut self) -> Result<Command, ()>;
    async fn respond_to_write(&mut self, buf: &mut [u8]) -> Result<(), ()>;
    async fn respond_to_read(&mut self, buf: &[u8]) -> Result<(), ()>;
}
