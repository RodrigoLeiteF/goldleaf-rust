use std::error::Error;
use std::fs::DirEntry;

use crate::filesystem;
use crate::command::Command;

pub fn GetFile(command: &mut Command) -> Result<(), Box<dyn Error>> {
    let path = command.read::<String>()?;
    let index = command.read::<i32>()?;

    let fixed_path = filesystem::normalize_path(&path);

    debug!("Requested directory: {:?} Index: {:?}", fixed_path, index);

    let files: Vec<DirEntry> = std::fs::read_dir(fixed_path)?
        .filter(|entry| {
            entry.as_ref().unwrap().path().is_file()
        })
        .map(|x| x.unwrap())
        .collect();

    let file: &DirEntry = files.get(index as usize).unwrap();
    let name = file
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
