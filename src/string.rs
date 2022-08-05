pub trait Padding<T> {
    fn with_padding(self, stringifier: fn(T) -> String, num: usize) -> String;
}

impl<T> Padding<T> for T {
    fn with_padding(self, stringifier: fn(T) -> String, num: usize) -> String {
        let mut s = stringifier(self);
        let len = s.len();
        if len < num {
            let d = num - len;
            s = "0".repeat(d) + &s;
        }
        s
    }
}

pub fn to_binary_string<T: std::fmt::Binary>(n: T) -> String {
    format!("{:b}", n)
}

pub fn to_hexadecimal_string<T: std::fmt::LowerHex>(n: T) -> String {
    format!("{:x}", n)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_padding() {
        assert_eq!(0b11u8.with_padding(to_binary_string, 8), "00000011");
    }
}
