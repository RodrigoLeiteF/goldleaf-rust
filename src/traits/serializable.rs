use byteorder::{ ReadBytesExt, WriteBytesExt, LittleEndian };
use std::convert::TryInto;
use std::io::Cursor;

pub trait Serializable<T> {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> T;
    fn write(cursor: &mut Cursor::<Vec<u8>>, byte: T) -> Result<(), std::io::Error>;
}

impl Serializable<String> for String {
    fn read(cursor: &mut Cursor::<Vec<u8>>) -> String {
        let length = cursor.read_i32::<LittleEndian>().expect("Could not read command id");

        let mut bytes = Vec::<u16>::with_capacity(512);
        for _b in 0..length {
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
