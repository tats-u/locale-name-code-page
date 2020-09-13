pub mod cp_table_type;
mod locale_to_cp_map;

use cp_table_type::CodePage;
use cp_table_type::TableNode::*;
use lazy_static::lazy_static;
use locale_to_cp_map::LOCALE_TO_CP_MAP;
use regex::Regex;
use std::borrow::Cow;
use std::convert::Into;

/// Suggests ANSI/OEM code pages used in Windows from the given locale name.
///
/// Locale names must be such as the following:
///
/// 1. It must consist of components that follows the regex `[a-zA-Z]+`.
/// 2. Its components must be joined by `-` or `_`. (e.g. `en-US`,`en_us`)  Single components (e.g. `en`) are also allowed.
/// 3. Joined locales (e.g. `en-US`) may be joined by `,`. (e.g. `en-US,ja_JP`)  The first valid locale will be used.
///
/// # Examples
///
/// ```
///  use locale_name_code_page::get_codepage;
///
/// if let Some(locale_en_us) = get_codepage("en-US") {
///   assert_eq!(locale_en_us.ansi, 1252);
///   assert_eq!(locale_en_us.oem, 437);
/// } else {
///    panic!("en-US must be supported.");
/// }
///
/// assert_eq!(get_codepage("ja"), get_codepage("ja_JP"));
/// assert_eq!(get_codepage("en-gb").unwrap().oem, 850);
/// assert_eq!(get_codepage("invalid_locale"), None);
/// assert_eq!(get_codepage("invalid_locale,en"), get_codepage("en_us"));
/// assert_eq!(get_codepage("ja,en"), get_codepage("ja"));
/// assert_eq!(get_codepage("JA_JP,en-us"), get_codepage("ja"));
/// ```
pub fn get_codepage<'a, S: Into<Cow<'a, str>>>(encoding: S) -> Option<&'static CodePage> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[-_]").unwrap();
        static ref VARIANT_REGEX: Regex = Regex::new(r".+=.+").unwrap();
    }
    for locale in encoding.into().split(',') {
        if VARIANT_REGEX.is_match(locale) {
            continue;
        }
        let mut current_map = &*LOCALE_TO_CP_MAP;
        let mut provisional_code_page = None;
        for token in RE.split(locale) {
            let token_lower = token.to_lowercase();
            match current_map.get(&*token_lower) {
                Some(WithCP(codepage, Some(next_ref))) => {
                    provisional_code_page = Some(codepage);
                    current_map = &next_ref;
                }
                Some(WithCP(codepage, None)) => {
                    provisional_code_page = Some(codepage);
                    break;
                }
                Some(WithoutCP(_)) => {
                    // current_map = next_ref; // not used
                    break;
                }
                None => break,
            }
        }

        if provisional_code_page.is_some() {
            return provisional_code_page;
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::cp_table_type::*;
    use super::*;
    use lazy_static::lazy_static;
    lazy_static! {
        static ref CP_EN_US: CodePage = CodePage::new(1252, 437);
        static ref CP_JA: CodePage = CodePage::new(932, 932);
        static ref CP_EN_GB: CodePage = CodePage::new(1252, 850);
        static ref TESTING_TARGET_SPLITTED: Vec<(Vec<&'static str>, &'static CodePage)> = vec![
            (vec!["en", "us"], &CP_EN_US),
            (vec!["en", "gb"], &CP_EN_GB),
            (vec!["ja", "jp"], &CP_JA),
            (vec!["ja"], &CP_JA)
        ];
        static ref TESTING_TARGET_JOINTED: Vec<(&'static str, &'static CodePage)> = vec![
            ("en-US", &CP_EN_US),
            ("en-GB", &CP_EN_GB),
            ("en_gb", &CP_EN_GB),
            ("ja-JP", &CP_JA),
            ("ja", &CP_JA),
            ("ja_JP.UTF-8", &CP_JA),
            ("en_lmfao", &CP_EN_US),
            ("en,ja", &CP_EN_US),
            ("en_us.UTF-8,ja_jp.EUC-JP", &CP_EN_US),
            ("en_us.UTF-8,ja_jp.EUC-JP", &CP_EN_US),
            ("aaaaaa,en", &CP_EN_US),
        ];
        static ref INVALID_TESTING_TARGET: Vec<&'static str> = vec!["aaa-bb", "aaa_bb", "aaa"];
    }
    #[test]
    fn codepage_string_test() {
        for (locale_ref, codepage_ref) in &*TESTING_TARGET_JOINTED {
            assert_eq!(get_codepage(*locale_ref), Some(*codepage_ref))
        }
    }

    #[test]
    fn invalid_string_test() {
        for str_ref in &*INVALID_TESTING_TARGET {
            assert_eq!(get_codepage(*str_ref), None);
        }
    }

    #[test]
    fn tree_tree_test() {
        for (tokens_ref, codepage_ref) in &*TESTING_TARGET_SPLITTED {
            let mut current_map = &*LOCALE_TO_CP_MAP;
            let mut provisional_code_page = None;
            for token in tokens_ref {
                match current_map.get(*token) {
                    Some(WithCP(codepage, Some(next_ref))) => {
                        provisional_code_page = Some(codepage);
                        current_map = next_ref;
                    }
                    Some(WithCP(codepage, None)) => {
                        provisional_code_page = Some(codepage);
                        break;
                    }
                    Some(WithoutCP(_)) => {
                        // current_map = next_ref; // not used
                        break;
                    }
                    None => panic!("Couldn't find node on `{}` in {:?}", *token, tokens_ref),
                }
            }
            assert_eq!(provisional_code_page, Some(*codepage_ref));
        }
    }
}
