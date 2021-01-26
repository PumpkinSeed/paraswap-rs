use ureq;
use serde::{Deserialize, Deserializer};
use core::fmt;
use serde::de::Unexpected;

const PRODUCTION_BASE_URL: &str = "https://api.paraswap.io";
const MOCK_SERVER_BASE_URL: &str = "https://private-anon-6d23491c3c-paraswapv2.apiary-mock.com";
const DEBUGGING_PROXY_BASE_URL: &str = "https://private-anon-6d23491c3c-paraswapv2.apiary-proxy.com";

pub trait Descriptor {
    fn list_tokens(self) -> Result<Tokens, ureq::Error>;
    fn get_rate(self);
    fn swap(self);
}

pub enum Mode {
    Production,
    MockServer,
    DebuggingProxy,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Decimals { String(String), U8(u8) }

#[derive(Deserialize, Debug)]
struct Token {
    symbol: String,
    address: String,
    decimals: Decimals,
    img: String,
    network: u8,
}

#[derive(Deserialize, Debug)]
struct Tokens {
    tokens: Vec<Token>,
}

pub struct Handler {
    mode: Mode,
}

impl Descriptor for Handler {
    fn list_tokens(self) -> Result<Tokens, ureq::Error> {
        const ENDPOINT: &str = "/v2/tokens";
        let tokens: Tokens = ureq::get(&self.url_builder(ENDPOINT)[..])
            .call()?
            .into_json()?;

        Ok(tokens)
    }

    fn get_rate(self) {
        const ENDPOINT: &str = "/v2/prices";
        unimplemented!()
    }

    fn swap(self) {
        unimplemented!()
    }
}

impl Handler {
    fn new() -> Handler {
        Handler{
            mode: Mode::MockServer,
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    fn url_builder(self, endpoint: &str) -> String {
        let base_url = match self.mode {
            Mode::Production => PRODUCTION_BASE_URL,
            Mode::MockServer => MOCK_SERVER_BASE_URL,
            Mode::DebuggingProxy => DEBUGGING_PROXY_BASE_URL,
        };

        format!("{}{}", base_url, endpoint)
    }
}

#[cfg(test)]
mod tests {
    use crate::paraswap::{Handler, Mode, Descriptor};

    #[test]
    fn test_list_tokens() {
        let mut para = Handler::new();
        para.set_mode(Mode::Production);
        para.list_tokens();
    }
}