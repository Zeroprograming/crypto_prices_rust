
use constants::ErrorResponse;
use reqwest::{self};
use std::{collections::HashMap};
use serde_json::{self, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CoinData {
    usd: f64
}

mod constants {
    #[derive(Debug)]
    pub struct ErrorResponse {
        pub error: String,
        pub status: String,
    }

    impl std::fmt::Display for ErrorResponse {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Error {}: {}", self.status, self.error)
        }
    }
}

#[tokio::main]
async fn main() {
    let mut coin: String = String::new();
    let result = std::io::stdin().read_line(&mut coin);

    match result {
        Ok(_) => {
            println!("Coin: {coin}");
            let price = get_price(&coin).await;
            match  price {
                Ok(price) => {
                    println!("Price: {}", price);
                }
                Err(error) => {
                    println!("Error: {}", error);
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

async fn get_price(_coin: &str) -> Result<String, ErrorResponse>{
    //Request to API
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", _coin);

    let response = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(error) => {
            println!("Error: {}", error);
            return Err(ErrorResponse {
                error: error.to_string(),
                status: "500".to_string(),
            });
        }
    };

    let json_response: String = match response.json().await {
        Ok(json) => json,
        Err(error) => {
            println!("Error: {}", error);
            return Err(ErrorResponse {
                error: error.to_string(),
                status: "500".to_string(),
            });
        }
    };

    println!("Struct: \n {:#?}", json_response);
    
    Err(ErrorResponse{
        error: "Coin not found".to_string(),
        status: "404".to_string(),
    })
    // return error
}
