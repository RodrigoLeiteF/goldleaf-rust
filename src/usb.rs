use std::{ slice, time::Duration, error::Error };
use rusb::{ Device, GlobalContext, DeviceHandle, DeviceDescriptor };
use crate::command::Command;

const READ_ENDPOINT: u8 = 129;
const WRITE_ENDPOINT: u8 = 0;

pub struct Interface {
    device: Device<GlobalContext>,
    descriptor: DeviceDescriptor,
    handle: DeviceHandle<GlobalContext>,
    product: String,
    serial_number: String,
}

impl Interface {
    pub fn try_new() -> Result<Self, Box<dyn Error>> {
	let device = Interface::find_nintendo_switch().expect("could not find nintendo switch");

	let mut handle = device.open()?;
	let descriptor = device.device_descriptor()?;

	// set active configuration so goldleaf can tell we want to communicate
	&handle.set_active_configuration(1)?;

	// claim the interface we're going to write to
	&handle.claim_interface(WRITE_ENDPOINT)?;

	let product = (&mut handle).read_product_string_ascii(&descriptor)?.to_string();
	let serial_number = (&mut handle).read_serial_number_string_ascii(&descriptor)?.to_string();

	Ok(Interface {
	    device,
	    descriptor,
	    handle,
	    product,
	    serial_number,
	})
    }

    pub fn wait_for_command(&self) -> Command {
	loop {
	    let mut vec = Vec::<u8>::with_capacity(512);
	    let buf =
		unsafe { slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };
	    
	    let timeout = Duration::from_secs(5);

	    if let Ok(len) = self.handle.read_bulk(READ_ENDPOINT, buf, timeout) {
		unsafe { vec.set_len(len) };

		let command = Command::new().read::<i32>(vec);
	    }
	}
    }

    fn find_nintendo_switch() -> Option<Device<GlobalContext>> {
	let devices = rusb::devices().unwrap();

	let device_match: Option<Device<GlobalContext>> = devices.iter().find(|device| {
	    let device_desc = device.device_descriptor().unwrap();

	    device_desc.vendor_id() == 0x057E && device_desc.product_id() == 0x3000
	});

	return device_match
    }
}
