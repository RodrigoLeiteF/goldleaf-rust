use std::error::Error;
use std::io::{ Cursor, Read };
use std::convert::{ TryInto, TryFrom };
use num_enum::{ IntoPrimitive, TryFromPrimitive };

use crate::filesystem;
use crate::traits::Serializable;
use crate::handlers;

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
            CommandIDs::GetDriveCount => handlers::GetDriveCount(command),
            CommandIDs::GetDriveInfo => handlers::GetDriveInfo(command),
            CommandIDs::GetDirectory => handlers::GetDirectory(command),
            CommandIDs::GetDirectoryCount => handlers::GetDirectoryCount(command),
            CommandIDs::GetFile => handlers::GetFile(command),
            CommandIDs::GetFileCount => handlers::GetFileCount(command),
            CommandIDs::StatPath => handlers::StatPath(command),
            _ => { debug!("No handler available for command: {:?}", resolved_command); Ok(()) },
        }
    }

}
