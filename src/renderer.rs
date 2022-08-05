use crate::parser::*;
use crate::string::{to_binary_string, to_hexadecimal_string, Padding};
use crate::unicode::*;

macro_rules! make_color {
    ($color: literal) => {
        concat!("\x1b[", $color, "m{}\x1b[0m")
    };
}

macro_rules! declare_color_function {
    ($fn:ident, $color: literal) => {
        fn $fn(s: String) -> String {
            format!(make_color!($color), s)
        }
    };
}

// declare_color_function!(black, "0;90");
declare_color_function!(red, "0;91");
declare_color_function!(green, "0;92");
// declare_color_function!(yellow, "0;93");
declare_color_function!(blue, "0;94");
declare_color_function!(purple, "0;95");
// declare_color_function!(cyan, "0;96");
declare_color_function!(white, "0;97");

static COLORIZERS: [fn(String) -> String; 4] = [red, green, blue, purple];

pub fn colorize(v: Vec<String>) -> String {
    let mut res = String::default();

    for (i, mut s) in v.into_iter().enumerate() {
        s = COLORIZERS[i](s.to_string());
        res = s + &res;
    }
    res
}

// each unit consists of 2 part, the 1*0 and the value
pub fn split_utf8_unit(unit: &[u8], len: usize) -> Vec<(String, String)> {
    let mut res = vec![];
    // layout msb, ... lsb
    let mask = UTF8HEADERMASK2[len - 1];
    let mut iter = unit.iter();
    // the first byte is special
    let msb = *iter.next().unwrap();
    res.push((
        mask.0.with_padding(to_binary_string, mask.1.into()),
        msb.with_padding(to_binary_string, (8 - mask.1).into()),
    ));

    for _ in 1..len {
        let u = *iter.next().unwrap();
        res.push((
            0b10u8.with_padding(to_binary_string, 2),
            u.with_padding(to_binary_string, 6),
        ));
    }
    res
}

// Renderer
pub fn render_bcp(unicode: u32, len: usize) -> (String, usize) {
    // example: "                  10000010101100"
    let mut str32 = format!("{unicode:32b}");

    let zone = UTF8BINARYCODEPOINTLENGH[len - 1];

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

    let len = f2.iter().map(|x| x.len()).sum();
    (colorize(f2), len)
}

pub fn colorize_pairs(pairs: Vec<(String, String)>) -> Vec<String> {
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

pub fn render_bu8(utf8_frags: &[u8], len: usize) -> String {
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

pub fn render_hu8(hu8: &[u8]) -> String {
    let len = hu8.len();
    let mut res = String::default();
    for (i, u) in hu8.iter().enumerate() {
        let mut colored = COLORIZERS[len - 1 - i](u.with_padding(to_hexadecimal_string, 2));
        colored.push(' ');
        res += &colored;
    }
    res = res.trim().to_string();
    res
}

// given a span of `len` size, place the s in the center of the span
fn align_text_center(s: &str, len: usize) -> (String, usize) {
    let slen = s.len();
    if slen >= len {
        return (s.to_string(), slen);
    }
    let spaces = len - slen;
    let left = spaces / 2;
    let right = spaces - left;

    (" ".repeat(left) + s + &" ".repeat(right), len)
}
fn align_text_left(s: &str, len: usize) -> (String, usize) {
    let slen = s.len();
    if slen >= len {
        return (s.to_string(), slen);
    }
    let spaces = len - slen;
    (s.to_string() + &" ".repeat(spaces), len)
}
fn align_text_right(s: &str, len: usize) -> (String, usize) {
    let slen = s.len();
    if slen >= len {
        return (s.to_string(), slen);
    }
    let spaces = len - slen;
    (" ".repeat(spaces) + s, len)
}

struct TextRepresentation {
    input: String,
    unicode: String,
    binary_code_point: String,
    binary_utf8: String,
    hex_utf8: String,
    binary_code_point_length: usize,
    binary_utf8_length: usize,
    hex_utf8_length: usize,
}

impl From<&Representaion> for TextRepresentation {
    fn from(rep: &Representaion) -> Self {
        let unicode = format!("U+{}", rep.unicode.with_padding(to_hexadecimal_string, 4));
        let (binary_code_point, binary_code_point_length) = render_bcp(rep.unicode, rep.hu8.len());
        let binary_utf8 = render_bu8(&rep.utf8_frags, rep.hu8.len());
        let hex_utf8 = render_hu8(&rep.hu8);
        let len = rep.hu8.len();
        let hex_utf8_length = len * 2 - 1;
        let binary_utf8_length = len * 8 + len - 1;
        TextRepresentation {
            input: rep.input.clone(),
            unicode,
            binary_code_point,
            binary_utf8,
            hex_utf8,
            binary_code_point_length,
            binary_utf8_length,
            hex_utf8_length,
        }
    }
}

pub fn render(rep: &Representaion) {
    let mut lines = vec![];
    // calculate the header width:
    let rep = TextRepresentation::from(rep);
    let mut spans = vec![];
    //  @ | U+
    let len = 1 + 1 + rep.unicode.len();

    let (c, len) = align_text_center("Character", len);
    spans.push(len);
    let len = rep.binary_code_point_length;

    let (bcp, len) = align_text_center("Binary code point", len);
    spans.push(len);
    let len = rep.binary_utf8_length;

    let (butf8, len) = align_text_center("Binary UTF-8", len);
    spans.push(len);
    let len = rep.hex_utf8_length;

    let (hutf8, len) = align_text_center("Hex UTF-8", len);
    spans.push(len);

    let total_span: usize = spans.iter().sum();
    let mut span = spans.iter();
    lines.push(format!(" {} {} {} {}", c, bcp, butf8, hutf8));
    lines.push("-".repeat(total_span + 10));
    lines.push(format!(
        "|{}|{}|{}|{}|{}|",
        rep.input,
        align_text_right(&rep.unicode, *span.next().unwrap() - 2).0,
        align_text_right(&rep.binary_code_point, *span.next().unwrap()).0,
        align_text_left(&rep.binary_utf8, *span.next().unwrap()).0,
        align_text_left(&rep.hex_utf8, *span.next().unwrap()).0
    ));

    for line in lines {
        println!("{}", line);
    }
}
