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
                println!("Price: {:?}", price_usd);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

async fn get_price(coin: &str) -> Result<f64, reqwest::Error> {
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        coin
    );


    let res: serde_json::Value = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    Ok(res[coin]["usd"].to_owned().as_f64().unwrap())
}
