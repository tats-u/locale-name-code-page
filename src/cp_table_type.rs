use ahash::AHashMap;

/// Struct that retains code page information
///
/// # Examples
///
/// ```
/// use locale_name_code_page::cp_table_type::CodePage;
/// let en_us = CodePage::new(1252, 437);
/// assert_eq!(en_us.ansi, 1252);
/// assert_eq!(en_us.oem, 437);
/// ```
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct CodePage {
    pub ansi: u16,
    pub oem: u16,
}

impl CodePage {
    /// Generate code page information instance
    ///
    /// # Examples
    ///
    /// ```
    /// use locale_name_code_page::cp_table_type::CodePage;
    /// let en_us = CodePage::new(1252, 437);
    /// assert_eq!(en_us.ansi, 1252);
    /// assert_eq!(en_us.oem, 437);
    /// ```
    pub fn new(ansi: u16, oem: u16) -> Self {
        return Self {
            ansi: ansi,
            oem: oem,
        };
    }
}

pub enum TableNode {
    WithCP(CodePage, Option<AHashMap<&'static str, TableNode>>),
    WithoutCP(AHashMap<&'static str, TableNode>),
}
