#![no_std]
#![cfg_attr(test, no_main)]

use rpi_pico as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
