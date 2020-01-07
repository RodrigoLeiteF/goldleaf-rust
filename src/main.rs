use std::{ process, slice, time::Duration, str::FromStr, io::Cursor };
use rusb::{ Device, GlobalContext, DeviceHandle };
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};

const READ_ENDPOINT: u8 = 129;
const WRITE_ENDPOINT: u8 = 0;

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn main() {
    let switch = match find_nintendo_switch() {
	Some(result) => result,
	None => {
	    println!("No Nintendo Switch found. Make sure Goldleaf is running.");
	    process::exit(126);
	}
    };

    let config = switch.active_config_descriptor().unwrap();
    let descriptor = switch.device_descriptor().unwrap();
    let mut handle = switch.open().unwrap();
    
    println!("Manufacturer: {:?} | Product: {:?} | Serial: {:?}",
	     handle.read_manufacturer_string_ascii(&descriptor).unwrap(),
	     handle.read_product_string_ascii(&descriptor).unwrap(),
	     handle.read_serial_number_string_ascii(&descriptor).unwrap(),
    );

    // Set active configuration so Goldleaf can tell we want to communicate
    handle.set_active_configuration(1).unwrap();

    // Claim the interface we're going to write to
    handle.claim_interface(0).expect("Could not claim read interface");

    read_command(handle);
}

fn read_command(handle: DeviceHandle<GlobalContext>) {
    let mut vec = Vec::<u8>::with_capacity(512);
    let buf =
	unsafe { slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };

    let timeout = Duration::from_secs(20);

    let len = handle.read_bulk(READ_ENDPOINT, buf, timeout).expect("Could not read");
    unsafe { vec.set_len(len) };

    let mut cursor = Cursor::new(&vec);

    println!("magic number: {:?}", cursor.read_i32::<LittleEndian>().unwrap());
    println!("command: {:?}", cursor.read_i32::<LittleEndian>().unwrap());
}

fn find_nintendo_switch() -> Option<Device<GlobalContext>> {
    let devices = rusb::devices().unwrap();

    let device_match: Option<Device<GlobalContext>> = devices.iter().find(|device| {
        let device_desc = device.device_descriptor().unwrap();

	device_desc.vendor_id() == 0x057E && device_desc.product_id() == 0x3000
    });

    device_match
}
