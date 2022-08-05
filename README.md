# REPC

REPC is a commandline tool written in Rust to display the **rep**resentations of **c**haracters

The table rendering is powered by [term-table-rs](https://github.com/RyanBluth/term-table-rs).

The output simulates the style of tables from [the UTF-8 entry of wikipedia](https://en.wikipedia.org/wiki/UTF-8).

## Usage

```shell
$ cargo run 😀

╔═════════════════╦═════════════════════════════╦═════════════════════════════════════╦═════════════╗
║    Character    ║      Binary code point      ║             Binary UTF-8            ║  Hex UTF-8  ║
╠═══════╦═════════╬═════════════════════════════╬═════════════════════════════════════╬═════════════╣
║   😀  ║ U+1f600 ║  0 0001 1111 0110 0000 0000 ║ 11110000 10011111 10011000 10000000 ║ f0 9f 98 80 ║
╚═══════╩═════════╩═════════════════════════════╩═════════════════════════════════════╩═════════════╝

$  cargo run $(print '\xee\x82\xb0')

╔═══════════════╦═════════════════════╦═════════════════════════════╦═══════════╗
║   Character   ║  Binary code point  ║         Binary UTF-8        ║ Hex UTF-8 ║
╠═════╦═════════╬═════════════════════╬═════════════════════════════╬═══════════╣
║    ║  U+e0b0 ║ 1110 0000 1011 0000 ║ 11101110 10000010 10110000  ║ ee 82 b0  ║
╚═════╩═════════╩═════════════════════╩═════════════════════════════╩═══════════╝
```
