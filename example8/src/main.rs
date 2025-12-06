// src/main.rs
mod i18n_service;
mod greeting_service;
mod api_gateway;

use api_gateway::ApiGateway;
use i18n_service::Lang;

fn main() {
    println!("{}", ApiGateway::request(Lang::default(), "hello"));
    println!("{}", ApiGateway::request(Lang::default(), "welcome"));
    println!("{}", ApiGateway::request(Lang::En, "bye"));
}