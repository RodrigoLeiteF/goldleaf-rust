#[macro_use] extern crate log;

use std::{ process, slice, time::Duration, error::Error };
use rusb::{ Device, GlobalContext, DeviceHandle };
use env_logger;

mod command;
mod usb;
mod filesystem;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let usb = usb::Interface::try_new()?;
    usb.wait_for_command();

    Ok(())
}
