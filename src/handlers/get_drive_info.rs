use sysinfo::{ SystemExt, DiskExt };
use std::error::Error;

use crate::command::Command;

pub fn GetDriveInfo(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let index = command.read::<i32>()?;
    let system: sysinfo::System = sysinfo::System::new();

    let disk = system.get_disks().get(index as usize).unwrap();

    let mount_point = disk.get_mount_point().to_str().unwrap().to_owned();
    let label = disk.get_name().to_str().unwrap().to_owned();

    debug!("Requested disk index: {:?} | Mount point: {:?} | Label {:?}",
           index, mount_point, label);

    command.response_start()?;
    command.write::<String>(label)?;
    command.write::<String>(mount_point)?;
    command.write::<i32>(0)?;
    command.write::<i32>(0)?;

    Ok(())
}

