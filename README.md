# Locale Name to Code Page for Rust

![CI (master)](<https://github.com/tats-u/locale-name-code-page/workflows/CI%20(master)/badge.svg>)
![CI (Release)](<https://github.com/tats-u/locale-name-code-page/workflows/CI%20(Release)/badge.svg>)
[![locale_name_code_page at crates.io](https://img.shields.io/crates/v/locale_name_code_page.svg)](https://crates.io/crates/locale_name_code_page)
[![locale_name_code_page at docs.rs](https://docs.rs/locale_name_code_page/badge.svg)](https://docs.rs/locale_name_code_page)
![Downloads (Crates.io)](https://img.shields.io/crates/d/locale_name_code_page)
![License (Crates.io)](https://img.shields.io/crates/l/locale_name_code_page)

This is a library that converts strings representing locale names to code pages that are used in Windows.

e.g.

- In `en-US` locale, Windows-1252 (code page id: `1252`) is used as the ANSI code page, and CP437 (code page id: `437`) is used as the OEM code page.
- In `ja-JP` locale, Shift_JIS (code page id: `932`) is used as both of the ANSI and OEM code pages.

## Usage

First, add `locale_name_code_page = "<2"` to your `Cargo.toml`.

```toml
[dependencies]
# *snip*
locale_name_code_page = "<2"
# *snip*
```

Then, convert strings representing locales to code pages like:

```rust
use locale_name_code_page::get_codepage;
use locale_name_code_page::cp_table_type::CodePage;

// IConverter has already been defined by you
fn get_converter_instance(codepage: &CodePage) -> Box<dyn IConverter> {
  // do something
  return Box::new(converter);
}

// *snip*

fn main() {
  // *snip*
  if let Some(codepage_ref) = get_codepage(locale_string) {
    let converter = get_converter_instance(codepage_ref);
    // *snip*
  } else {
    eprintln!("Error: {} doesn't represent a valid locale.", locale_string);
    std::process::exit(1);
  }
}
```

Obtained codepage (instance of `locale_name_code_page::cp_table_type::CodePage`) can be used as follows:

```rust
use locale_name_code_page::get_codepage;

fn main() {
  let en_cp = get_codepage("en-US").unwrap();
  // prints "en-US locale: 1252 (ANSI) / 437 (OEM)"
  println!("en-US locale: {} (ANSI) / {} (OEM)", en_cp.ansi, en_cp.oem);
}
```

## Source of Information

https://web.archive.org/web/20180104073254/https://www.microsoft.com/resources/msdn/goglobal/default.mspx

## FAQ

### How can I convert codepage to encoder/decoder?

Use the following libraries:

#### ANSI encodings (including CJKV languages)

Combine with [codepage](https://crates.io/crates/codepage) and [encoding_rs](https://crates.io/crates/encoding_rs).

#### OEM encodings (except for CJKV languages)

Use [oem_cp](https://crates.io/crates/oem_cp).

### How can I get the current locale?

Use [locale_config](https://crates.io/crates/locale_config).

### I want to port this library to other languages.

You can use `assets/nls_info.json` in your automatic code generation script.

## LICENSE

MIT
