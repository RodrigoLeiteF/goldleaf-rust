use std::{ convert::TryFrom, convert::TryInto, error::Error };
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use num_enum::{ IntoPrimitive, TryFromPrimitive };
use sysinfo::{ProcessExt, SystemExt};

const INPUT_MAGIC_NUMBER: i32 = 0x49434C47;
const OUTPUT_MAGIC_NUMBER: i32 = 0x4F434C47;

pub struct Command {
    pub id: Option<i64>,
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
    
    pub fn read<T: Serializable>(&mut self) -> Result<i64, &'static str> {
        self.magic_number = Some(i32::read(&mut self.input_cursor).try_into().unwrap());

        if self.magic_number.unwrap() == 0 {
            return Ok(0)
        }

        if self.magic_number.unwrap() != INPUT_MAGIC_NUMBER {
            return Err("Magic number doesn't match that of Goldleaf")
        }

        Ok(T::read(&mut self.input_cursor))
    }

    pub fn write<T: Serializable>(&mut self, data: i64) -> Result<(), std::io::Error> {
        T::write(&mut self.output_cursor, data)
    }

    pub fn handle(&mut self, command_id: i32) -> Result<Vec::<u8>, Box<dyn Error>> {
        println!("Command id: {:?}", command_id);
        let command = CommandIDs::try_from(command_id).expect("Unrecognized command");
        println!("Handling command: {:?}", command);

        match command.handle(self) {
            Ok(_) => println!("Handled!"),
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
        match self {
            CommandIDs::Invalid => { println!("Invalid command!"); Ok(()) },
            CommandIDs::GetDriveCount => self.GetDriveCount(command),
            _ => { println!("No handler available for command command: {:?}", command.id.unwrap()); Ok(()) },
        }
    }

    fn GetDriveCount(&self, command: &mut Command) -> Result<(), Box<dyn Error>> {
        let mut system: sysinfo::System = sysinfo::System::new();
        let drives = system.get_disks().into_iter().count();

        command.response_start()?;
        command.write::<i32>(drives.try_into()?)?;

        Ok(())
    }
}

pub trait Serializable {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i64;
    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i64) -> Result<(), std::io::Error>;
}

impl Serializable for i32 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i64 {
        cursor.read_i32::<LittleEndian>().expect("Could not read command id").into()
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i64) -> Result<(), std::io::Error> {
        cursor.write_i32::<LittleEndian>(byte.try_into().unwrap())
    }
}

impl Serializable for i16 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i64 {
        cursor.read_i16::<LittleEndian>().expect("Could not read command id").into()
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i64) -> Result<(), std::io::Error> {
        cursor.write_i16::<LittleEndian>(byte.try_into().unwrap())
    }
}

impl Serializable for i64 {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> i64 {
        cursor.read_i64::<LittleEndian>().expect("Could not read command id")
    }

    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: i64) -> Result<(), std::io::Error> {
        cursor.write_i64::<LittleEndian>(byte)
    }
}
