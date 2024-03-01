use std::io::{Cursor, Read};

pub fn parse_i32(cursor: &mut Cursor<&[u8]>) -> i32 {
    let mut i32_bytes: [u8; 4] = [0; 4];
    cursor.read_exact(&mut i32_bytes).expect("parse i32 error");
    i32::from_le_bytes(i32_bytes)

}
pub fn parse_i8(cursor: &mut Cursor<&[u8]>) -> i8 {
    let mut i8_bytes: [u8; 1] = [0; 1];
    cursor.read_exact(&mut i8_bytes).expect("parse i8 error");
    i8::from_le_bytes(i8_bytes)
}

pub fn parse_u32(cursor: &mut Cursor<&[u8]>) -> u32 {
    let mut u32_bytes = [0; 4];
    cursor.read_exact(&mut u32_bytes).expect("parse u32 error");
    u32::from_le_bytes(u32_bytes)
}

pub fn parse_u16(cursor: &mut Cursor<&[u8]>) -> u16 {
    let mut u16_bytes = [0; 2];
    cursor.read_exact(&mut u16_bytes).expect("parse u16 error");
    u16::from_le_bytes(u16_bytes)
}

pub fn parse_u8(cursor: &mut Cursor<&[u8]>) -> u8 {
    let mut u8_bytes = [0; 1];
    cursor.read_exact(&mut u8_bytes).expect("parse u8 error");
    u8::from_le_bytes(u8_bytes)
}

pub fn parse_f32(cursor: &mut Cursor<&[u8]>) -> f32 {
    let mut f32_bytes = [0; 4];
    cursor.read_exact(&mut f32_bytes).expect("parse f32 error");
    f32::from_le_bytes(f32_bytes)
}
