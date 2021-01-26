#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Decimals {
    String(String),
    U8(u8),
}

#[derive(Deserialize, Debug)]
pub struct Token {
    symbol: String,
    address: String,
    decimals: Decimals,
    img: String,
    network: u8,
}

#[derive(Deserialize, Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

enum SwapSide {
    BUY,  //= "BUY",
    SELL, // = 'SELL',
}

pub struct RateRequest {
    pub src_token: String,  // from
    pub dest_token: String, // to
    pub amount: String,     // amount
                            //pub side: SwapSide,
                            // RateOptions
}

pub struct Rates {
    #[serde(rename(deserialize = "priceRoute"))]
    price_route: PriceRoute,
}

pub struct PriceRoute {
    #[serde(rename(deserialize = "srcAmount"))]
    src_amount: String,
    best_routes: Vec<BestRoute>,
    others: Vec<Other>,
    block_number: String,
    details: Details,
    from_usd: String,
    to_usd: String,
    price_with_slippage: String,
    best_route_gas_cost_usd: String,
    best_route_gas: String,
    multi_route: Vec<String>,
    amount: String,
    amount_with_fee: String,
    to_usd_with_fee: String,
}

pub struct BestRoute {
    exchange: String,
    src_amount: String,
    percent: u16,
    data: Data,
    amount: String,
    amount_with_fee: String,
}

pub struct Data {
    token_from: String,
    token_to: String,
    path: Vec<String>,
    swaps: Vec<Swap>,
    gas_usd: String,
}

pub struct Other {
    exchange: String,
    rate: String,
    unit: String,
    data: Data,
    rate_with_fee: String,
    unit_with_fee: String,
}

pub struct Swap {
    pool: String,
    token_in_param: String,
    token_out_param: String,
    max_price: String,
}

pub struct Details {
    token_from: String,
    token_to: String,
    src_amount: String,
    amount: String,
}
