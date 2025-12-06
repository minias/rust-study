// FILE: src/i18n_service.rs
#[derive(Clone, Copy, Debug)]
pub enum Lang {
    Ko,
    En,
}

impl Lang {
    pub fn default() -> Self {
        Self::Ko
    }
}

pub struct I18nService;

impl I18nService {
    pub fn translate(lang: Lang, key: &str) -> &'static str {
        match (lang, key) {
            (Lang::Ko, "hello") => "안녕하세요!",
            (Lang::Ko, "welcome") => "환영합니다!",
            (Lang::Ko, "bye") => "안녕히 가세요!",
            (Lang::En, "hello") => "Hello!",
            (Lang::En, "welcome") => "Welcome!",
            (Lang::En, "bye") => "Goodbye!",
            _ => "[알 수 없는 문자열 키]",
        }
    }
}
