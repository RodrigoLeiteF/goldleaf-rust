#[macro_use] extern crate log;

use std::error::Error;
use env_logger;

mod command;
mod usb;
mod filesystem;
mod traits;
mod handlers;

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
