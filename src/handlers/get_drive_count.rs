use std::convert::TryInto;
use sysinfo::SystemExt;
use std::error::Error;

use crate::command::Command;

pub fn GetDriveCount(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let system: sysinfo::System = sysinfo::System::new();
    let drives = system.get_disks().into_iter().count();

    debug!("Found {:?} drives", drives);

    command.response_start()?;
    command.write::<i32>(drives.try_into()?)?;

    Ok(())
}
