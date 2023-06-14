use constants::ErrorResponse;
use reqwest::{self};
use std::collections::HashMap;

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
    loop {
        let mut coin: String = String::new();
        println!("Enter a coin name (or 'exit' to quit):");
        std::io::stdin().read_line(&mut coin).unwrap();
        let coin = coin.trim().to_lowercase();

        if coin == "exit" {
            println!("Exiting...");
            break;
        }

        let price = get_price(&coin).await;
        match price {
            Ok(price_usd) => {
                println!("Price: {:?}", price_usd.parse::<f64>().unwrap());
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

async fn get_price(coin: &str) -> Result<String, ErrorResponse> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        coin
    );

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

    let json_response: HashMap<String, HashMap<String, f64>> = match response.json().await {
        Ok(json) => json,
        Err(error) => {
            println!("Error: {}", error);
            return Err(ErrorResponse {
                error: error.to_string(),
                status: "500".to_string(),
            });
        }
    };

    for (_key, value) in &json_response {
        if let Some(price_data) = value.get("usd") {
            return Ok(price_data.to_string());
        }
    }

    Err(ErrorResponse {
        error: "Coin not found".to_string(),
        status: "404".to_string(),
    })
}
