use crate::types;
use ureq;

const PRODUCTION_BASE_URL: &str = "https://api.paraswap.io";
const MOCK_SERVER_BASE_URL: &str = "https://private-anon-6d23491c3c-paraswapv2.apiary-mock.com";
const DEBUGGING_PROXY_BASE_URL: &str =
    "https://private-anon-6d23491c3c-paraswapv2.apiary-proxy.com";

pub trait Descriptor {
    fn list_tokens(self) -> Result<types::Tokens, ureq::Error>;
    fn get_rate(self, req: types::RateRequest);
    fn swap(self);
}

pub enum Mode {
    Production,
    MockServer,
    DebuggingProxy,
}

pub struct Handler {
    mode: Mode,
    network: u8, // NOTE: That's u8 for now
}

impl Descriptor for Handler {
    fn list_tokens(self) -> Result<types::Tokens, ureq::Error> {
        let endpoint: &str = &format!("/v2/tokens/{}", self.network)[..];
        let tokens: types::Tokens = ureq::get(&self.url_builder(endpoint)[..])
            .call()?
            .into_json()?;

        Ok(tokens)
    }

    fn get_rate(self, req: types::RateRequest) {
        let endpoint: &str = "/v2/prices";
        let res: String = ureq::get(&self.url_builder(endpoint)[..])
            .query("from", &req.src_token[..])
            .query("to", &req.dest_token[..])
            .query("amount", &req.amount[..])
            .call()
            .unwrap()
            .into_string()
            .unwrap();
        println!("{}", res);
    }

    fn swap(self) {
        unimplemented!()
    }
}

impl Handler {
    pub fn new() -> Handler {
        Handler {
            mode: Mode::MockServer,
            network: 1,
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_network(&mut self, network: u8) {
        self.network = network;
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
    use crate::paraswap::{Descriptor, Handler, Mode};
    use crate::types;

    #[test]
    fn test_list_tokens() {
        let mut para = Handler::new();
        para.set_mode(Mode::Production);
        let tokens = para.list_tokens().unwrap();
        println!("{:?}", tokens);
    }

    #[test]
    fn test_get_rate() {
        let mut para = Handler::new();
        para.set_mode(Mode::Production);
        let tokens = para.get_rate(types::RateRequest {
            src_token: String::from("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"),
            dest_token: String::from("0x6b175474e89094c44da98b954eedeac495271d0f"),
            amount: String::from("100000000000"),
        });
        println!("{:?}", tokens);
    }
}
