// FILE: src/api_gateway.rs
use crate::greeting_service::GreetingService;
use crate::i18n_service::Lang;

pub struct ApiGateway;

impl ApiGateway {
    pub fn request(lang: Lang, action: &str) -> &'static str {
        let svc = GreetingService::new(lang);
        match action {
            "hello" => svc.hello(),
            "welcome" => svc.welcome(),
            "bye" => svc.bye(),
            _ => "[알 수 없는 API 요청]",
        }
    }
}
