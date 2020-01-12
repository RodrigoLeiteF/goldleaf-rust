use std::error::Error;
use std::convert::TryInto;

use crate::filesystem;
use crate::command::Command;

pub fn GetDirectoryCount(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let path = command.read::<String>()?;

    let fixed_path = filesystem::normalize_path(&path);
    debug!("Requested path: {:?} | Fixed path: {:?}", path, fixed_path);

    let directory_count = std::fs::read_dir(&fixed_path)?
        .filter(|entry| entry
                .as_ref()
                .unwrap()
                .path()
                .is_dir())
        .count();

    debug!("Found {:?} directories in path {:?}", directory_count, fixed_path);

    command.response_start()?;
    command.write::<i32>(directory_count.try_into().expect("Could not convert to i32 for some reason"))?;

    Ok(())
}
