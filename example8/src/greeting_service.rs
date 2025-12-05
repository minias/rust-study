// src/greeting_service.rs
use crate::i18n_service::{I18nService, Lang};

pub struct GreetingService {
    lang: Lang,
}

impl GreetingService {
    pub fn new(lang: Lang) -> Self {
        Self { lang }
    }

    pub fn hello(&self) -> &'static str {
        I18nService::translate(self.lang, "hello")
    }
    pub fn welcome(&self) -> &'static str {
        I18nService::translate(self.lang, "welcome")
    }
    pub fn bye(&self) -> &'static str {
        I18nService::translate(self.lang, "bye")
    }
}
