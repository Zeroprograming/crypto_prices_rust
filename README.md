# Crypto Prices
This is a Rust project that fetches the current price of cryptocurrencies using the CoinGecko API. It allows you to enter the name of a cryptocurrency and retrieves its price in USD.

## Prerequisites
Rust (version 2021 or later)
Cargo (Rust's package manager)
## Installation
Clone the repository: git clone https://github.com/Zeroprograming/crypto_prices_rust.git
Navigate to the project directory: cd crypto-prices
Build the project using Cargo: cargo build
## Usage
Run the program: cargo run
Enter the name of a cryptocurrency when prompted. You can also type "exit" to quit the program.
The program will fetch the current price of the entered cryptocurrency from the CoinGecko API and display it.
## Dependencies
This project uses the following dependencies:

reqwest: A Rust HTTP client for making API requests.
tokio: An asynchronous runtime for executing asynchronous operations in Rust.
serde: A Rust library for serializing and deserializing data structures.
serde_json: A crate for working with JSON data in Rust.
These dependencies are specified in the Cargo.toml file and will be automatically installed when building the project.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.
