use core::ops::Range;

pub(crate) fn nvram_read(_address: usize) -> u32 {
    0
}

pub(crate) fn nvram_write(_address: usize, _value: u32) {}

pub(crate) fn nvram_valid_range() -> Range<usize> {
    0..0
}
