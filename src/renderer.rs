use crate::parser::*;

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
pub fn red(s: String) -> String {
    format!(BIRED!(), s)
}

pub fn green(s: String) -> String {
    format!(BIGREEN!(), s)
}

pub fn blue(s: String) -> String {
    format!(BIBLUE!(), s)
}

pub fn purple(s: String) -> String {
    format!(BIPURPLE!(), s)
}

pub fn white(s: String) -> String {
    format!(BIWHITE!(), s)
}

static COLORIZERS: [fn(String) -> String; 4] = [red, green, blue, purple];

pub fn colorize(v: Vec<String>) -> String {
    let mut res = String::default();

    for (i, mut s) in v.into_iter().enumerate() {
        s = COLORIZERS[i](s.to_string());
        res = s + &res;
    }
    res
}

// Renderer
pub fn render_bcp(unicode: u32, len: usize) -> String {
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
        let mut colored = COLORIZERS[len - 1 - i](u.with_padding_to(to_hexadecimal_string, 2));
        colored.push(' ');
        res += &colored;
    }
    res = res.trim().to_string();
    res
}
