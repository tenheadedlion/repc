use std::{env, fmt::Display, slice};

#[derive(Debug, Default)]
struct Representaion {
    input: String,
    // binary code point
    bcp: Vec<String>,
    // binary utf-8
    bu8: Vec<String>,
    // hexadecimal utf-8
    hu8: Vec<String>,
    // unicode code: U+xxxx
    unicode: String,
}

// Renderer
fn render_bu8(bu8: &[String]) -> String {
    let mut res = String::from("");
    for i in bu8.iter() {
        res += i;
    }
    res
}
fn render_bcp(bu8: &[String]) -> String {
    let mut res = String::from("");
    for i in bu8.iter() {
        res += i;
    }
    res
}
fn render_hu8(bu8: &[String]) -> String {
    let mut res = String::from("");
    for i in bu8.iter() {
        res += i;
    }
    res
}

impl Display for Representaion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "|{i}|{u}|{bcp}|{bu8}|{hu8}|",
            i = self.input,
            u = self.unicode,
            bcp = render_bcp(&self.bcp),
            bu8 = render_bu8(&self.bu8),
            hu8 = render_hu8(&self.hu8)
        )
    }
}

trait Padding<T: Sized> {
    fn with_padding(self) -> Self;
}

// pad with 0 if lengh is less than 8n
impl Padding<String> for String {
    fn with_padding(self) -> Self {
        let len = self.len();
        let mood = len % 8;
        if mood != 0 {
            let rest = if len < 8 {
                8 - len
            } else {
                let times: usize = len / 8 + 1;
                times * 8 - len
            };
            return "0".repeat(rest) + &self;
        }
        self
    }
}

/*
      11000010 will left shift 6 bytes to be prepended to the least significant byte
11000010______
the most significant byte of Rust String is placed at index 0
*/
static UTF8PATTERN: [(u8, u8); 4] = [
    (0b00000000, 0),
    (0b11000000, 6),
    (0b11100000, 6 * 2),
    (0b11110000, 6 * 3),
];

#[derive(Debug, Clone)]
struct RepcError;

fn utf8_to_bcp_unicode(input: &str) -> Result<(Vec<u8>, u32), RepcError> {
    let mut flat: Vec<u32> = vec![];
    let mut iter = input.bytes();
    let len = input.len();
    // the most significant byte is stored at index 0
    let msb = iter.next().ok_or(RepcError)?;
    dbg!(format!("{msb:b}"));
    let mut binary = (UTF8PATTERN[len - 1].0 ^ msb) as u32;
    binary <<= UTF8PATTERN[len - 1].1;
    dbg!(binary);
    flat.push(binary);

    for (i, c) in iter.enumerate() {
        dbg!(i);
        let mut binary: u32 = (c ^ 0b10000000) as u32;
        binary <<= 6 * (len - (i + 2));
        flat.push(binary);
    }

    let mut unicode: u32 = 0;
    for f in flat {
        unicode |= f;
    }
    let mut bcp = u32_as_u8(unicode);
    bcp.reverse();
    Ok((bcp, unicode))
}

// the result is small endian, the least significant byte is at index 0
fn u32_as_u8(src: u32) -> Vec<u8> {
    let ptr = &src as *const _;
    let res = unsafe { slice::from_raw_parts(ptr as *mut u8, 4) };
    res.to_vec()
}

fn decode(input: &str) -> Result<Representaion, RepcError> {
    // fragments scattered in bytes
    let (bcp_u32, unicode_u32) = utf8_to_bcp_unicode(input)?;
    let hu8_u32: Vec<u8> = input.bytes().collect();
    let unicode = format!("U+{:x}", unicode_u32);

    let hu8 = hu8_u32
        .iter()
        .map(|hex| -> String { format!("{hex:x}") })
        .collect();

    let bu8 = hu8_u32
        .iter()
        .map(|hex| -> String { format!("{hex:b}") })
        .collect();

    let bcp = bcp_u32
        .iter()
        .map(|bin| -> String { format!("{bin:b}").with_padding() })
        .collect();

    Ok(Representaion {
        input: input.to_string(),
        bu8,
        hu8,
        bcp,
        unicode,
    })
}

fn main() -> Result<(), RepcError> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        print!("Usage: repc [char]");
        return Err(RepcError);
    }
    let input = &args[1];
    if input.eq("-h") || input.eq("--help") {
        print!("Usage: repc [char]");
        return Err(RepcError);
    }
    let rep = decode(input)?;
    dbg!(&rep);
    print!("{}", rep);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_padding() {
        assert_eq!("11".to_string().with_padding(), "00000011");
        assert_eq!("111111111".to_string().with_padding(), "0000000111111111");
    }

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
            let (binary, unicode) = utf8_to_bcp_unicode(ch).unwrap();
            assert_eq!(
                &binary as &[u8],
                [0b0100_1000, 0b0000_0011, 0b0000_0001, 0b0000_0000]
            );
            assert_eq!(unicode, 0x10348);
        }
        {
            let ch = "£";
            let (binary, unicode) = utf8_to_bcp_unicode(ch).unwrap();
            assert_eq!(
                &binary as &[u8],
                [0b1010_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x00a3);
        }
        {
            let ch = "$";
            let (binary, unicode) = utf8_to_bcp_unicode(ch).unwrap();
            assert_eq!(
                &binary as &[u8],
                [0b010_0100, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x0024);
        }
    }
}
