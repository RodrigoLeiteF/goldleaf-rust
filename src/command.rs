use log::Level;

use std::{ convert::TryFrom, convert::TryInto, error::Error };
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::fs::DirEntry;
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use num_enum::{ IntoPrimitive, TryFromPrimitive };
use sysinfo::{ProcessExt, SystemExt, DiskExt};

const INPUT_MAGIC_NUMBER: i32 = 0x49434C47;
const OUTPUT_MAGIC_NUMBER: i32 = 0x4F434C47;

pub struct Command {
    pub id: Option<i32>,
    pub magic_number: Option<i32>,

    input_cursor: Cursor<Vec::<u8>>,
    output_cursor: Cursor<Vec::<u8>>,
}

impl Command {
    pub fn new(input_buffer: Vec::<u8>) -> Self {
        Command {
            id: None,
            magic_number: None,
            output_cursor: Cursor::new(Vec::<u8>::new()),
            input_cursor: Cursor::new(input_buffer),
        }
    }
    
    pub fn read<T: Serializable<T>>(&mut self) -> Result<T, &'static str> {
        match self.magic_number {
            None => self.magic_number = Some(i32::read(&mut self.input_cursor).try_into().unwrap()),
            Some(_) => {},
        };

        if self.magic_number.unwrap() != INPUT_MAGIC_NUMBER {
            return Err("Magic number doesn't match that of Goldleaf")
        }

        Ok(T::read(&mut self.input_cursor))
    }

    pub fn write<T: Serializable<T>>(&mut self, data: T) -> Result<(), std::io::Error> {
        T::write(&mut self.output_cursor, data)
    }

    pub fn handle(&mut self, command_id: i32) -> Result<Vec::<u8>, Box<dyn Error>> {
        let command = CommandIDs::try_from(command_id).expect("Unrecognized command");
        debug!("Handling command: {:?} / ID: {:?}", command, command_id);

        match command.handle(self) {
            Ok(_) => debug!("Handled successfully"),
            Err(e) => return Err(e),
        }

        let mut out = Vec::<u8>::with_capacity(4096);
        self.output_cursor.set_position(0);
        self.output_cursor.read_to_end(&mut out)?;

        // Fill the rest of the vector I guess?
        out.resize(4096, 0);

        Ok(out)
    }

    pub fn response_start(&mut self) -> Result<(), std::io::Error> {
        self.write::<i32>(OUTPUT_MAGIC_NUMBER.into())?;
        self.write::<i32>(0)?;

        Ok(())
    }
}

#[derive(Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
enum CommandIDs {
    Invalid = 0,
    GetDriveCount = 1,
    GetDriveInfo = 2,
    StatPath = 3,
    GetFileCount = 4,
    GetFile = 5,
    GetDirectoryCount = 6,
    GetDirectory = 7,
    StartFile = 8,
    ReadFile = 9,
    WriteFile = 10,
    EndFile = 11,
    Create = 12,
    Delete = 13,
    Rename = 14,
    GetSpecialPathCount = 15,
    GetSpecialPath = 16,
    SelectFile = 17,
}

impl CommandIDs {
    fn handle(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let resolved_command = CommandIDs::try_from(command.id.unwrap()).expect("Unrecognized command");

        match self {
            CommandIDs::Invalid => { debug!("Invalid command received"); Ok(()) },
            CommandIDs::GetDriveCount => self.GetDriveCount(command),
            CommandIDs::GetDriveInfo => self.GetDriveInfo(command),
            CommandIDs::GetDirectory => self.GetDirectory(command),
            CommandIDs::GetDirectoryCount => self.GetDirectoryCount(command),
            CommandIDs::GetFileCount => self.GetFileCount(command),
            CommandIDs::StatPath => self.StatPath(command),
            _ => { debug!("No handler available for command: {:?}", resolved_command); Ok(()) },
        }
    }

    fn GetDriveCount(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let mut system: sysinfo::System = sysinfo::System::new();
        let drives = system.get_disks().into_iter().count();

        debug!("Found {:?} drives", drives);

        command.response_start()?;
        command.write::<i32>(drives.try_into()?)?;

        Ok(())
    }

    fn GetDriveInfo(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
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

    fn GetDirectoryCount(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let path = command.read::<String>()?;

        let fixed_path = path.split(":").collect::<Vec<&str>>()[0];
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

    fn GetFileCount(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let path = command.read::<String>()?;

        let fixed_path = path.replace(":", "");
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

    fn GetDirectory(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let path = command.read::<String>()?;
        let index = command.read::<i32>()?;

        let fixed_path = path.replace(":", "");
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

    fn StatPath(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let path = command.read::<String>()?;

        let fixed_path = path.replace(":", "");

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

}

pub trait Serializable<T> {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> T;
    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: T) -> Result<(), std::io::Error>;
}

impl Serializable<String> for String {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> String {
        let length = cursor.read_i32::<LittleEndian>().expect("Could not read command id");

        let mut bytes = Vec::<u16>::with_capacity(512);
        for b in 0..length {
            let byte = cursor.read_u16::<LittleEndian>().unwrap();
            bytes.push(byte);
        }

        String::from_utf16(&bytes).unwrap()
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: String) -> Result<(), std::io::Error> {
        cursor.write_i32::<LittleEndian>(byte.len().try_into().unwrap()).unwrap(); // Wow this is ugly

        Ok(for b in byte.as_bytes() {
            cursor.write_u16::<LittleEndian>(b.to_owned().into())?;
        })
    }
}

impl Serializable<u8> for u8 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> u8 {
        cursor.read_u8().expect("Could not read byte")
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: u8) -> Result<(), std::io::Error> {
        cursor.write_u8(byte.try_into().unwrap())
    }
}

impl Serializable<i32> for i32 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i32 {
        cursor.read_i32::<LittleEndian>().expect("Could not read command id")
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i32) -> Result<(), std::io::Error> {
        cursor.write_i32::<LittleEndian>(byte.try_into().unwrap())
    }
}

impl Serializable<i16> for i16 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i16 {
        cursor.read_i16::<LittleEndian>().expect("Could not read command id")
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i16) -> Result<(), std::io::Error> {
        cursor.write_i16::<LittleEndian>(byte.try_into().unwrap())
    }
}

impl Serializable<i64> for i64 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i64 {
        cursor.read_i64::<LittleEndian>().expect("Could not read command id")
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i64) -> Result<(), std::io::Error> {
        cursor.write_i64::<LittleEndian>(byte)
    }
}
