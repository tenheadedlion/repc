use std::{
    env,
    fmt::{format, Display},
    slice,
};

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

#[derive(Debug, Default)]
struct Representaion {
    input: String,
    // hexadecimal utf-8
    hu8: Vec<u8>,
    // unicode code: U+xxxx
    unicode: u32,
}

static UTF8BINARYCODEPOINTLENGH: [[usize; 4]; 4] =
    [[7, 0, 0, 0], [6, 5, 0, 0], [6, 6, 4, 0], [6, 6, 6, 3]];
macro_rules! BIBLACK {
    () => {
        "\x1b[0;90m{}\x1b[0m"
    };
}
macro_rules! BIRED {
    () => {
        "\x1b[0;91m{}\x1b[0m"
    };
}
macro_rules! BIGREEN {
    () => {
        "\x1b[0;92m{}\x1b[0m"
    };
}
macro_rules! BIYELLOW {
    () => {
        "\x1b[0;93m{}\x1b[0m"
    };
}
macro_rules! BIBLUE {
    () => {
        "\x1b[0;94m{}\x1b[0m"
    };
}
macro_rules! BIPURPLE {
    () => {
        "\x1b[0;95m{}\x1b[0m"
    };
}
macro_rules! BICYAN {
    () => {
        "\x1b[0;96m{}\x1b[0m"
    };
}
macro_rules! BIWHITE {
    () => {
        "\x1b[0;97m{}\x1b[0m"
    };
}

// Fix: replace the repeat
fn red(s: String) -> String {
    format!(BIRED!(), s)
}

fn green(s: String) -> String {
    format!(BIGREEN!(), s)
}

fn blue(s: String) -> String {
    format!(BIBLUE!(), s)
}

fn purple(s: String) -> String {
    format!(BIPURPLE!(), s)
}

static COLORIZERS: [fn(String) -> String; 4] = [red, green, blue, purple];

fn colorize(v: Vec<String>) -> String {
    let mut res = String::default();

    for (i, mut s) in v.into_iter().enumerate() {
        s = COLORIZERS[i](s.trim().to_string());
        res = s + &res;
    }
    res
}

// Renderer
fn render(unicode: u32, hu8: &[u8]) -> String {
    // len | effective numbers width
    // 1   | 7        = 7
    // 2   | 5 6      = 11
    // 3   | 4 6 6    = 16
    // 4   | 3 6 6 6  = 21

    let mut len = hu8.len();
    let mut res = format!("{unicode:32b}");
    dbg!(&res);

    let zone = UTF8BINARYCODEPOINTLENGH[len - 1];
    dbg!(zone);
    let mut f: Vec<String> = vec![];
    let mut cnt = 0;
    for z in zone {
        let mut zstr = String::default();
        for _ in 0..z {
            if cnt % 4 == 0 {
                zstr.push(' ');
            }
            // pop() can't be None in this scenario
            let mut c = res.pop().unwrap();
            if c == ' ' {
                c = '0';
            }
            zstr.push(c);
            cnt += 1;
        }
        zstr = zstr.chars().rev().collect();
        f.push(zstr);
    }
    dbg!(&f);
    println!("{}", colorize(f));

    let a = format!(BIYELLOW!(), "r");
    println!("{}{}", a, a);

    dbg!(len);
    dbg!(&res);
    dbg!(&res.len());

    dbg!(&res);
    res
}
fn render_bu8(bu8: &[u8]) -> String {
    String::default()
}
fn render_hu8(bu8: &[u8]) -> String {
    String::default()
}

impl Display for Representaion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "|{i}|{u}|{bcp}|{bu8}|{hu8}|",
            i = self.input,
            u = self.unicode,
            bcp = render(self.unicode, &self.hu8),
            bu8 = render_bu8(&self.hu8),
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
#[derive(Debug, Clone)]
struct RepcError;

fn utf8_to_unicode(input: &str) -> Result<u32, RepcError> {
    let mut flat: Vec<u32> = vec![];
    let mut iter = input.bytes();
    let len = input.len();
    // the most significant byte is stored at index 0
    let msb = iter.next().ok_or(RepcError)?;
    let mut binary = (UTF8PATTERN[len - 1].0 ^ msb) as u32;
    binary <<= UTF8PATTERN[len - 1].1;
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
    Ok(unicode)
}

// the result is little endian, the least significant byte is at index 0
#[allow(dead_code)]
fn u32_as_u8(src: u32) -> Vec<u8> {
    let ptr = &src as *const _;
    let res = unsafe { slice::from_raw_parts(ptr as *mut u8, 4) };
    res.to_vec()
}

fn decode(input: &str) -> Result<Representaion, RepcError> {
    // fragments scattered in bytes
    let unicode = utf8_to_unicode(input)?;
    let hu8: Vec<u8> = input.bytes().collect();

    Ok(Representaion {
        input: input.to_string(),
        hu8,
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
            let unicode = utf8_to_unicode(ch).unwrap();
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b0100_1000, 0b0000_0011, 0b0000_0001, 0b0000_0000]
            );
            assert_eq!(unicode, 0x10348);
        }
        {
            let ch = "£";
            let unicode = utf8_to_unicode(ch).unwrap();
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b1010_0011, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x00a3);
        }
        {
            let ch = "$";
            let unicode = utf8_to_unicode(ch).unwrap();
            let binary = u32_as_u8(unicode);
            assert_eq!(
                &binary as &[u8],
                [0b010_0100, 0b0000_0000, 0b0000_0000, 0b0000_0000]
            );
            assert_eq!(unicode, 0x0024);
        }
    }
}
