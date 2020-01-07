use std::{ process, slice, time::Duration, error::Error };
use rusb::{ Device, GlobalContext, DeviceHandle };

mod command;
mod usb;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn main() -> Result<(), Box<dyn Error>> {
    let usb = usb::Interface::try_new()?;
    usb.wait_for_command();

    Ok(())
}

// fn read_command(handle: DeviceHandle<GlobalContext>) {
    // let mut vec = Vec::<u8>::with_capacity(512);
    // let buf =
	// unsafe { slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };
// 
    // let timeout = Duration::from_secs(20);
// 
    // let len = handle.read_bulk(READ_ENDPOINT, buf, timeout).expect("Could not read");
    // unsafe { vec.set_len(len) };
// 
    // let command = command::Command::new().read::<i32>(vec);
// }
