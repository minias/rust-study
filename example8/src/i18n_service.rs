// --------------------------------------------------------------------
// rust-study/example7 (MSA style) - 파일 분리 버전
// --------------------------------------------------------------------
// 아래는 실제 파일 구조 기준으로 분리한 코드 예시입니다.
// 프로젝트 구조 예:
// rust-study/example7/
// ├── src/
// │ ├── i18n_service.rs
// │ ├── greeting_service.rs
// │ ├── api_gateway.rs
// │ └── main.rs
// --------------------------------------------------------------------

// ====================================================================
// FILE: src/i18n_service.rs
// ====================================================================
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
