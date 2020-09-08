#![no_std]
#![feature(never_type)]
#![feature(ptr_offset_from)]
#![feature(maybe_uninit)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_as_ptr)]
#![cfg_attr(feature = "set_panic_handler", feature(lang_items))]

extern crate byteorder;
extern crate crc;
extern crate embedded_hal;
extern crate riot_sys;

pub mod error;

#[cfg(riot_module_saul)]
pub mod saul;
#[cfg(riot_module_shell)]
pub mod shell;
pub mod stdio;
pub mod thread;
// internally cfg-gated as it has a no-op implementation
#[cfg(riot_module_gcoap)]
pub mod gcoap;
#[cfg(riot_module_gnrc)]
pub mod gnrc;
#[cfg(riot_module_gnrc)]
pub mod gnrc_util;
pub mod i2c;
#[cfg(riot_module_core_msg)]
pub mod msg;

#[cfg(riot_module_periph_spi)]
pub mod spi;

#[cfg(riot_module_periph_adc)]
pub mod adc;

#[cfg(riot_module_xtimer)]
pub mod delay;
pub mod mutex;
#[cfg(riot_module_pthread)]
pub mod rwlock;

#[cfg(feature = "set_panic_handler")]
mod panic;

#[cfg(feature = "with_coap_message")]
pub mod coap_message;
#[cfg(feature = "with_coap_handler")]
pub mod coap_handler;

pub mod interrupt;
pub mod main;
