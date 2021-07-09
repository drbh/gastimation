extern crate reqwest;
extern crate colorize;

use crate::colorize::AnsiColor;
use chrono;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockNativeGasEstimateResponse {
    pub pending_block_number_val: i64,
    pub seconds: i64,
    pub max_price: i64,
    pub estimated_transactions: i64,
    pub estimated_prices: Vec<EstimatedPrice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EstimatedPrice {
    pub confidence: i64,
    pub price: i64,
}

#[tokio::main]
async fn main() {
    loop {
        let mut headers = header::HeaderMap::new();
        headers.insert("Connection", "keep-alive".parse().unwrap());
        headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".parse().unwrap());
        headers.insert("Accept", "*/*".parse().unwrap());
        headers.insert("Sec-GPC", "1".parse().unwrap());
        headers.insert("Origin", "https://www.blocknative.com".parse().unwrap());
        headers.insert("Sec-Fetch-Site", "cross-site".parse().unwrap());
        headers.insert("Sec-Fetch-Mode", "cors".parse().unwrap());
        headers.insert("Sec-Fetch-Dest", "empty".parse().unwrap());
        headers.insert("Referer", "https://www.blocknative.com/".parse().unwrap());
        headers.insert("Accept-Language", "en-US,en;q=0.9".parse().unwrap());

        let res = reqwest::Client::new()
            .get("https://blocknative-api.herokuapp.com/data")
            .headers(headers)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let _v: BlockNativeGasEstimateResponse = serde_json::from_str(&res).expect("bad message");

        let padded_block = format!(" Block: {} ", _v.pending_block_number_val);
        let padded_string_time = format!(
            " {} ",
            chrono::offset::Utc::now().format("%Y-%m-%d %H:%M:%S")
        );
        let padded_max_price = format!(" Max: {} ", _v.max_price);

        let estimate_1 = format!(
            " {}% ➨ {} ",
            _v.estimated_prices[0].confidence, _v.estimated_prices[0].price
        );
        let estimate_2 = format!(
            " {}% ➨ {} ",
            _v.estimated_prices[1].confidence, _v.estimated_prices[1].price
        );
        let estimate_3 = format!(
            " {}% ➨ {} ",
            _v.estimated_prices[2].confidence, _v.estimated_prices[2].price
        );
        let estimate_4 = format!(
            " {}% ➨ {} ",
            _v.estimated_prices[3].confidence, _v.estimated_prices[3].price
        );
        let estimate_5 = format!(
            " {}% ➨ {} ",
            _v.estimated_prices[4].confidence, _v.estimated_prices[4].price
        );

        println!("\n");
        println!("{}", padded_block.black().bold().b_cyanb());
        println!("{}", padded_string_time.black().bold().b_magentab());
        println!("{}", estimate_1.yellow());
        println!("{}", estimate_2.yellow());
        println!("{}", estimate_3.yellow());
        println!("{}", estimate_4.yellow());
        println!("{}", estimate_5.yellow());
        println!("{}", padded_max_price.black().bold().b_yellowb());

        thread::sleep(Duration::from_millis(5000))
    }
}
