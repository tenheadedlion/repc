use std::{env, fmt::Display, slice, vec};

/*
      11000010 will left shift 6 bytes to be prepended to the least significant byte
11000010______
the most significant byte of Rust String is placed at index 0
*/
static UTF8HEADERMASK: [(u8, u8); 4] = [
    (0b00000000, 0),
    (0b11000000, 6),
    (0b11100000, 6 * 2),
    (0b11110000, 6 * 3),
];

// proceeding bits, and their number
static UTF8HEADERMASK2: [(u8, usize); 4] = [(0b0, 1), (0b110, 3), (0b1110, 4), (0b11110, 5)];

#[derive(Debug, Default)]
struct Representaion {
    input: String,
    // hexadecimal utf-8
    hu8: Vec<u8>,
    // unicode code: U+xxxx
    unicode: u32,
    utf8_frags: Vec<u8>,
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

fn white(s: String) -> String {
    format!(BIWHITE!(), s)
}

static COLORIZERS: [fn(String) -> String; 4] = [red, green, blue, purple];

fn colorize(v: Vec<String>) -> String {
    let mut res = String::default();

    for (i, mut s) in v.into_iter().enumerate() {
        s = COLORIZERS[i](s.to_string());
        res = s + &res;
    }
    res
}

// Renderer
fn render_bcp(unicode: u32, len: usize) -> String {
    // example: "                  10000010101100"
    let mut str32 = format!("{unicode:32b}");
    dbg!(&str32);

    let zone = UTF8BINARYCODEPOINTLENGH[len - 1];
    dbg!(zone);
    let mut f: Vec<String> = vec![];
    // should not start with 0
    let mut cnt = 1;
    for z in zone {
        let mut zstr = String::default();
        // walk through the zone
        for _ in 0..z {
            // pop() can't be None in this scenario
            let mut c = str32.pop().unwrap();
            if c == ' ' {
                c = '0';
            }
            // this will result in a reverse version of str32
            // "0011 01"
            zstr.push(c);
            if cnt % 4 == 0 {
                zstr.push(' ');
            }
            cnt += 1;
        }
        dbg!(&zstr);
        // reversed toL "10 1100"
        zstr = zstr.chars().rev().collect();
        f.push(zstr);
    }
    let mut f2 = f
        .into_iter()
        .filter(|s| -> bool { !s.is_empty() })
        .collect::<Vec<String>>();
    // the last one should be trimmed, because it happens to be mod 4
    //          but at the same time the end of string
    let t = f2.last_mut().unwrap();
    *t = t.trim().to_string();
    dbg!(&f2);
    colorize(f2)
}

fn colorize_pairs(pairs: Vec<(String, String)>) -> Vec<String> {
    let pn = pairs.len();
    let mut res = vec![];
    for (i, pair) in pairs.into_iter().enumerate() {
        let mut j = white(pair.0);
        // i = 0 -> len - 1
        // i = 1 -> len - 1 - 1
        j += &COLORIZERS[pn - 1 - i](pair.1);
        res.push(j);
    }
    res
}

// each unit consists of 2 part, the 1*0 and the value
fn split_utf8_unit(unit: &[u8], len: usize) -> Vec<(String, String)> {
    let mut res = vec![];
    // layout msb, ... lsb
    let mask = UTF8HEADERMASK2[len - 1];
    let mut iter = unit.iter();
    // the first byte is special
    let msb = iter.next().unwrap();
    res.push((
        mask.0.with_padding_to(to_binary_string, mask.1),
        msb.with_padding_to(to_binary_string, 8 - mask.1),
    ));

    for _ in 1..len {
        let u = *iter.next().unwrap();
        res.push((
            0b10u8.with_padding_to(to_binary_string, 2),
            u.with_padding_to(to_binary_string, 6),
        ));
    }
    res
}

fn render_bu8(utf8_frags: &[u8], len: usize) -> String {
    let pairs = split_utf8_unit(utf8_frags, len);
    let strs = colorize_pairs(pairs);
    let mut res = String::default();
    for mut s in strs {
        s.push(' ');
        res += &s;
    }
    res = res.trim_end().to_string();
    res
}

fn render_hu8(hu8: &[u8]) -> String {
    let len = hu8.len();
    let mut res = String::default();
    for (i, u) in hu8.iter().enumerate() {
        let mut colored = COLORIZERS[len - 1 - i](u.with_padding_to(to_hexadecimal_string, 2));
        colored.push(' ');
        res += &colored;
    }
    res = res.trim().to_string();
    res
}

impl Display for Representaion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "│{i}│U+{u:x}│{bcp}│{bu8}│{hu8}│",
            i = self.input,
            u = self.unicode,
            bcp = render_bcp(self.unicode, self.hu8.len()),
            bu8 = render_bu8(&self.utf8_frags, self.hu8.len()),
            hu8 = render_hu8(&self.hu8)
        )
    }
}

trait Padding {
    fn with_padding(self) -> String;
    fn with_padding_to(self, stringifier: fn(u8) -> String, num: usize) -> String;
}

impl Padding for u8 {
    fn with_padding_to(self, stringifier: fn(u8) -> String, num: usize) -> String {
        let mut s = stringifier(self);
        let len = s.len();
        if len < num {
            let d = num - len;
            s = "0".repeat(d) + &s;
        }
        s
    }
    fn with_padding(self) -> String {
        self.with_padding_to(to_binary_string, 8)
    }
}

fn to_binary_string(n: u8) -> String {
    format!("{:b}", n)
}

fn to_hexadecimal_string(n: u8) -> String {
    format!("{:x}", n)
}

#[derive(Debug, Clone)]
struct RepcError;

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
        dbg!(i);
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
fn u32_as_u8(src: u32) -> Vec<u8> {
    let ptr = &src as *const _;
    let res = unsafe { slice::from_raw_parts(ptr as *mut u8, 4) };
    res.to_vec()
}

fn decode(input: &str) -> Result<Representaion, RepcError> {
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
        assert_eq!(0b11u8.with_padding(), "00000011");
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
