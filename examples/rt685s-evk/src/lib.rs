#![no_std]

use defmt_rtt as _;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use panic_probe as _;

#[link_section = ".otfad"]
#[used]
static OTFAD: [u8; 256] = [0; 256];

#[link_section = ".fcb"]
#[used]
static FCB: FlexSPIFlashConfigurationBlock = FlexSPIFlashConfigurationBlock::build();

#[link_section = ".biv"]
#[used]
static BOOT_IMAGE_VERSION: u32 = 0x01000000;

#[link_section = ".keystore"]
#[used]
static KEYSTORE: [u8; 2048] = [0; 2048];

pub fn delay(cycles: usize) {
    for _ in 0..cycles {
        cortex_m::asm::nop();
    }
}
