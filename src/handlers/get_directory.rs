use std::error::Error;
use std::fs::DirEntry;

use crate::command::Command;
use crate::filesystem;

pub fn GetDirectory(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let path = command.read::<String>()?;
    let index = command.read::<i32>()?;

    let fixed_path = filesystem::normalize_path(&path);
    let directories: Vec<DirEntry> = std::fs::read_dir(fixed_path)?
        .filter(|entry| {
            entry.as_ref().unwrap().path().is_dir()
        })
        .map(|x| x.unwrap())
        .collect();

    let directory: &DirEntry = directories.get(index as usize).unwrap();
    let name = directory
        .path()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    debug!("Found directory {:?} in {:?}", name, path);

    command.response_start()?;
    command.write::<String>(name)?;

    Ok(())
}
