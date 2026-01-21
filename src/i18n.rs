/// Internationalization module for multi-language support
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Language {
    #[default]
    English,
    Chinese,
}

impl Language {
    pub fn current() -> Self {
        Language::English
    }
}

pub struct I18n;

impl I18n {
    pub fn get(key: &str, lang: Language) -> &'static str {
        match lang {
            Language::English => Self::get_en(key),
            Language::Chinese => Self::get_zh(key),
        }
    }

    fn get_en(key: &str) -> &'static str {
        match key {
            "file" => "File",
            _ => "Unknown",
        }
    }

    fn get_zh(key: &str) -> &'static str {
        match key {
            "file" => "文件",
            _ => "未知",
        }
    }
}
