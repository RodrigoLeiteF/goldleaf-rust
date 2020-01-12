use std::error::Error;
use std::convert::TryInto;

use crate::filesystem;
use crate::command::Command;

pub fn StatPath(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let path = command.read::<String>()?;

    let fixed_path = filesystem::normalize_path(&path);

    debug!("Requested path: {:?}", fixed_path);

    let path_buf = std::path::PathBuf::from(fixed_path);

    command.response_start()?;

    if path_buf.is_dir() {
        command.write::<i32>(2)?;
        command.write::<i64>(0)?;
    } else {
        let size = path_buf.metadata().unwrap().len().try_into().unwrap();

        command.write::<i32>(1)?;
        command.write::<i64>(size)?;
    }

    Ok(())
}
