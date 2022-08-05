// number of the effective bits for UTF8 character of different lengths
pub static UTF8BINARYCODEPOINTLENGH: [[usize; 4]; 4] =
    [[7, 0, 0, 0], [6, 5, 0, 0], [6, 6, 4, 0], [6, 6, 6, 3]];

// proceeding bits, and their number
pub static UTF8HEADERMASK2: [(u8, usize); 4] = [(0b0, 1), (0b110, 3), (0b1110, 4), (0b11110, 5)];

/*
      11000010 will left shift 6 bytes to be prepended to the least significant byte
11000010______
the most significant byte of Rust String is placed at index 0
*/
pub static UTF8HEADERMASK: [(u8, u8); 4] = [
    (0b00000000, 0),
    (0b11000000, 6),
    (0b11100000, 6 * 2),
    (0b11110000, 6 * 3),
];
