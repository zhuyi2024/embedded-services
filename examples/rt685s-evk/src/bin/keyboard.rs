#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_imxrt;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let _p = embassy_imxrt::init(Default::default());

    info!("Platform initialization complete ...");

    loop {
        embedded_services_examples::delay(10_000_000);
    }
}
