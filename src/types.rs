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

#[derive(Deserialize, Debug)]
pub struct Rates {
    #[serde(rename(deserialize = "priceRoute"))]
    price_route: PriceRoute,
}

#[derive(Deserialize, Debug)]
pub struct PriceRoute {
    #[serde(rename(deserialize = "srcAmount"))]
    src_amount: String,

    #[serde(rename(deserialize = "bestRoute"))]
    best_routes: Vec<BestRoute>,

    others: Vec<Other>,

    #[serde(rename(deserialize = "blockNumber"))]
    block_number: u64,

    details: Details,

    #[serde(rename(deserialize = "fromUSD"))]
    from_usd: String,

    #[serde(rename(deserialize = "toUSD"))]
    to_usd: String,

    #[serde(rename(deserialize = "priceWithSlippage"))]
    price_with_slippage: String,

    #[serde(rename(deserialize = "bestRouteGasCostUSD"))]
    best_route_gas_cost_usd: String,

    #[serde(rename(deserialize = "bestRouteGas"))]
    best_route_gas: String,

    #[serde(rename(deserialize = "multiRoute"))]
    multi_route: Vec<String>,

    amount: String,

    #[serde(rename(deserialize = "amountWithFee"))]
    amount_with_fee: String,

    #[serde(rename(deserialize = "toUSDWithFee"))]
    to_usd_with_fee: String,
}

#[derive(Deserialize, Debug)]
pub struct BestRoute {
    exchange: String,

    #[serde(rename(deserialize = "srcAmount"))]
    src_amount: String,

    percent: u16,

    data: Data,

    amount: String,

    #[serde(rename(deserialize = "amountWithFee"))]
    amount_with_fee: String,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename(deserialize = "tokenFrom"))]
    token_from: String,

    #[serde(rename(deserialize = "tokenTo"))]
    token_to: String,

    path: Option<Vec<String>>,

    swaps: Option<Vec<Swap>>,

    #[serde(rename(deserialize = "gasUSD"))]
    gas_usd: String,
}

#[derive(Deserialize, Debug)]
pub struct Other {
    exchange: String,

    rate: String,

    unit: String,

    data: Data,

    #[serde(rename(deserialize = "rateWithFee"))]
    rate_with_fee: String,

    #[serde(rename(deserialize = "unitWithFee"))]
    unit_with_fee: String,
}

#[derive(Deserialize, Debug)]
pub struct Swap {
    pool: String,

    #[serde(rename(deserialize = "tokenInParam"))]
    token_in_param: String,

    #[serde(rename(deserialize = "tokenOutParam"))]
    token_out_param: String,

    #[serde(rename(deserialize = "maxPrice"))]
    max_price: String,
}

#[derive(Deserialize, Debug)]
pub struct Details {
    #[serde(rename(deserialize = "tokenFrom"))]
    token_from: String,

    #[serde(rename(deserialize = "tokenTo"))]
    token_to: String,

    #[serde(rename(deserialize = "srcAmount"))]
    src_amount: String,

    amount: String,
}

#[cfg(test)]
mod tests {
    use crate::types;

    #[test]
    fn test_get_rate() {
        let data = r#"{"priceRoute":{"srcAmount":"100000000000","bestRoute":[{"exchange":"Uniswap","srcAmount":"100000000000","percent":100,"data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","gasUSD":"6.903072"},"amount":"129393119207424","amountWithFee":"129393119207424"}],"others":[{"exchange":"Uniswap","rate":"129393119207589","unit":"1288738828497448900000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","gasUSD":"6.903072"},"rateWithFee":"129393119207589","unitWithFee":"1288738828497448900000"},{"exchange":"Bancor","rate":"164027279705108","unit":"106935634005594640000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","path":["0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","0xb1CD6e4153B2a390Cf00A6556b0fC1458C4A5533","0x1F573D6Fb3F13d689FF844B4cE37794d79a7FF1C","0xd1146B08e8104EeDBa44a73B7bda1d102c6ceDC9","0x309627af60F0926daa6041B8279484312f2bf060","0xcb913ED43e43cc7Cec1D77243bA381615101E7E4","0x6B175474E89094C44Da98b954EedeAC495271d0F"],"gasUSD":"60.833322"},"rateWithFee":"164027279705108","unitWithFee":"106935634005594640000"},{"exchange":"Kyber","rate":"127990601318001","unit":"1279906013180019600000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","gasUSD":"51.77304"},"rateWithFee":"127990601318001","unitWithFee":"1279906013180019600000"},{"exchange":"Balancer","rate":"132818793814031","unit":"1292512332292169700000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","swaps":[{"pool":"0x1159907045848ed563a7ab883d2abe75b02d4723","tokenInParam":"100000000000","tokenOutParam":"0","maxPrice":"115792089237316195423570985008687907853269984665640564039457584007913129639935"}],"gasUSD":"34.51536"},"rateWithFee":"132818793814031","unitWithFee":"1292512332292169700000"},{"exchange":"UniswapV2","rate":"128454328102351","unit":"1284527765805508900000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","path":["0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2","0x6B175474E89094C44Da98b954EedeAC495271d0F"],"gasUSD":"11.217492"},"rateWithFee":"128454328102351","unitWithFee":"1284527765805508900000"},{"exchange":"SushiSwap","rate":"128373769371162","unit":"1283719649434522000000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","path":["0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2","0x6B175474E89094C44Da98b954EedeAC495271d0F"],"gasUSD":"11.217492"},"rateWithFee":"128373769371162","unitWithFee":"1283719649434522000000"},{"exchange":"DefiSwap","rate":"128100923768922","unit":"1279148985788497100000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","path":["0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2","0x6B175474E89094C44Da98b954EedeAC495271d0F"],"gasUSD":"11.217492"},"rateWithFee":"128100923768922","unitWithFee":"1279148985788497100000"},{"exchange":"ParaSwapPool","rate":"128080000000000","unit":"1280800000000000000000","data":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","gasUSD":"15.531912"},"rateWithFee":"128080000000000","unitWithFee":"1280800000000000000000"}],"blockNumber":11731747,"details":{"tokenFrom":"0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE","tokenTo":"0x6B175474E89094C44Da98b954EedeAC495271d0F","srcAmount":"100000000000","amount":"129393119207424"},"fromUSD":"0","toUSD":"0","priceWithSlippage":"128099188015349","bestRouteGasCostUSD":"16.549597389600002","bestRouteGas":"191794","multiRoute":[],"amount":"129393119207424","amountWithFee":"129393119207424","toUSDWithFee":"0.0000000000"}}"#;
        let p: types::Rates = serde_json::from_str(data).unwrap();
        println!("{:?}", p);
    }
}

//
