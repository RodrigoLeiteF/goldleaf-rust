use log::Level;

use std::{ slice, time::Duration, error::Error, convert::TryInto };
use rusb::{ Device, GlobalContext, DeviceHandle, DeviceDescriptor };
use crate::command::Command;

const READ_ENDPOINT: u8 = 0x81;
const INTERFACE: u8 = 0x0;
const WRITE_ENDPOINT: u8 = 0x1;

pub struct Interface {
    device: Device<GlobalContext>,
    descriptor: DeviceDescriptor,
    handle: DeviceHandle<GlobalContext>,
    product: String,
    serial_number: String,
}

impl Interface {
    pub fn try_new() -> Result<Self, Box<dyn Error>> {
        debug!("Looking for USB Device");
        let device = Interface::find_nintendo_switch().expect("could not find nintendo switch");

        debug!("USB Device found. Initializing...");
        let mut handle = device.open()?;
        let descriptor = device.device_descriptor()?;


        debug!("Setting the active configuration");
        // set active configuration so goldleaf can tell we want to communicate
        &handle.set_active_configuration(1)?;

        debug!("Claiming the interface");
        // claim the interface we're going to write to
        &handle.claim_interface(INTERFACE)?;

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
        println!("Listening for commands...");
        loop {
            let mut vec = Vec::with_capacity(4096);
            let buf =
                unsafe { slice::from_raw_parts_mut((&mut vec[..]).as_mut_ptr(), vec.capacity()) };
            
            let timeout = Duration::from_secs(30);

            if let Ok(len) = self.handle.read_bulk(READ_ENDPOINT, buf, timeout) {
                unsafe { vec.set_len(len) };

                debug!("Received command: {:?}", &vec[0..128]);

                let mut command = Command::new(vec);
                command.id = Some(command.read::<i32>().unwrap());

                debug!("Handling command: {:?}", command.id);
                let response = command.handle(command.id.unwrap().try_into().unwrap()).unwrap();

                debug!("Writing response (truncated): {:?}", &response[0..128]);
                let wrote_bytes = self.handle.write_bulk(WRITE_ENDPOINT, &response[..], timeout).unwrap();
                debug!("Wrote {:?} bytes", wrote_bytes);
            }
        }
    }

    fn find_nintendo_switch() -> Option<Device<GlobalContext>> {
        let devices = rusb::devices().unwrap();
        debug!("Found {:?} USB devices", devices.len());

        let device_match: Option<Device<GlobalContext>> = devices.iter().find(|device| {
            let device_desc = device.device_descriptor().unwrap();

            device_desc.vendor_id() == 0x057E && device_desc.product_id() == 0x3000
        });

        return device_match
    }
}
