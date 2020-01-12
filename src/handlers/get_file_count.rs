use std::error::Error;
use std::convert::TryInto;

use crate::filesystem;
use crate::command::Command;

pub fn GetFileCount(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let path = command.read::<String>()?;

    let fixed_path = filesystem::normalize_path(&path);
    debug!("Requested path: {:?} | Fixed path: {:?}", path, fixed_path);

    let file_count = std::fs::read_dir(&fixed_path)?
        .filter(|entry| entry
                .as_ref()
                .unwrap()
                .path()
                .is_file())
        .count();

    debug!("Found {:?} files in path {:?}", file_count, fixed_path);

    command.response_start()?;
    command.write::<i32>(file_count.try_into().expect("Could not convert to i32 for some reason"))?;

    Ok(())
}
