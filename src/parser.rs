use crate::error::*;
use crate::unicode::*;

use std::{slice, vec};

#[derive(Debug, Default)]
pub struct Representaion {
    pub input: String,
    // hexadecimal utf-8
    pub hu8: Vec<u8>,
    // unicode code: U+xxxx
    pub unicode: u32,
    pub utf8_frags: Vec<u8>,
}

fn utf8_to_unicode(input: &str) -> Result<(u32, Vec<u8>), RepcError> {
    let mut flat: Vec<u32> = vec![];
    // the real value that fill the holes of unicode format
    let mut reals: Vec<u8> = vec![];
    let mut iter = input.bytes();
    let len = input.len();
    // the most significant byte is stored at index 0
    let msb = iter.next().ok_or(RepcError)?;
    let real = UTF8HEADERMASK[len - 1].0 ^ msb;
    let mut binary = real as u32;
    reals.push(real);
    binary <<= UTF8HEADERMASK[len - 1].1;
    flat.push(binary);

    for (i, c) in iter.enumerate() {
        let real = c ^ 0b10000000;
        let mut binary: u32 = real as u32;
        reals.push(real);
        binary <<= 6 * (len - (i + 2));
        flat.push(binary);
    }

    let mut unicode: u32 = 0;
    for f in flat {
        unicode |= f;
    }
    Ok((unicode, reals))
}

// the result is little endian, the least significant byte is at index 0
#[allow(dead_code)]
fn u32_as_u8(src: u32) -> Vec<u8> {
    let ptr = &src as *const _;
    let res = unsafe { slice::from_raw_parts(ptr as *mut u8, 4) };
    res.to_vec()
}

pub fn decode(input: &str) -> Result<Representaion, RepcError> {
    // fragments scattered in bytes
    let (unicode, utf8_frags) = utf8_to_unicode(input)?;
    let hu8: Vec<u8> = input.bytes().collect();

    Ok(Representaion {
        input: input.to_string(),
        hu8,
        unicode,
        utf8_frags,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_u32_to_u8() {
        let unicode = 0b0000_0000_0000_0001_0000_0011_0100_1000;
        let binary = u32_as_u8(unicode);
        assert_eq!(
            &binary as &[u8],
            [0b0100_1000, 0b0000_0011, 0b0000_0001, 0b0000_0000]
        );
        let hex_utf8: u32 = 0xf0908d88;
        let bin_utf8 = u32_as_u8(hex_utf8);
        assert_eq!(
            &bin_utf8 as &[u8],
            [0b1000_1000, 0b1000_1101, 0b1001_0000, 0b1111_0000]
        )
    }

    #[test]
    fn test_utf8_to_u8() {
        {
            let ch = "𐍈";
            let unicode = utf8_to_unicode(ch).unwrap().0;
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b0100_1000, 0b0000_0011, 0b0000_0001, 0b0000_0000]
            );
            assert_eq!(unicode, 0x10348);
        }
        {
            let ch = "€";
            let unicode = utf8_to_unicode(ch).unwrap().0;
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b1010_1100, 0b0010_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x20ac);
        }
        {
            let ch = "£";
            let unicode = utf8_to_unicode(ch).unwrap().0;
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b1010_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x00a3);
        }
        {
            let ch = "$";
            let unicode = utf8_to_unicode(ch).unwrap().0;
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b010_0100, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x0024);
        }
    }
}
