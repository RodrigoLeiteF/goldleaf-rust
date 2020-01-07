use std::{ io::Cursor, convert::TryFrom, convert::TryInto, error::Error };
use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

const INPUT_MAGIC_NUMBER: i32 = 0x49434C47;
const OUTPUT_MAGIC_NUMBER: i32 = 0x4F434C47;

pub struct Command {
    pub id: Option<i64>,
    pub magic_number: Option<i32>,

    output_cursor: Cursor<Vec::<u8>>,
}

impl Command {
    pub fn new() -> Self {
        Command {
            id: None,
            magic_number: None,
            output_cursor: Cursor::new(Vec::<u8>::new()),
        }
    }
    
    pub fn read<T: Serializable>(&mut self, input_buffer: Vec::<u8>) -> Result<i64, &'static str> {
        let mut cursor = Cursor::new(input_buffer);

        self.magic_number = Some(i32::read(&mut cursor).try_into().unwrap());

        if self.magic_number.unwrap() != INPUT_MAGIC_NUMBER {
            return Err("Magic number doesn't match that of Goldleaf")
        }

        Ok(T::read(&mut cursor))
    }

    pub fn write<T: Serializable>(&mut self, data: i64) -> Result<(), std::io::Error> {
        T::write(&mut self.output_cursor, data)
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
    fn handle(&self, command: Command) -> Result<(), Box<dyn Error>> {
        // Do something
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
