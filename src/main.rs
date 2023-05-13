use std::fs::File;
use std::io::prelude::*;

use axum::{
    routing::get,
    http::{Response, StatusCode},
    response::Html,
    Router,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct StockVec {
    stock: Vec<[String; 2]>,
}

#[tokio::main]
async fn main() {
    let yaml_content = std::fs::read_to_string("stock.yaml").expect("Failed to read file");
    let stock_data: StockVec = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");
    
    // Set up the Axum web server
    let app = Router::new().route("/", get(get_values));

    // Start the web server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_values() -> Html<String> {
    let api_key = "IUWE71WEWZMLHPJK";
    let symbol = "AAPL"; // APPLE company symbol
    let client = Client::new();
    let endpoint: String = format!(
        "https://www.alphavantage.co/query?function=INCOME_STATEMENT&symbol={}&apikey={}",
        symbol, api_key
    );

    match client.get(&endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let data = response.text().await.unwrap();
                //let values = parse_values_from_response(&data);

                // Generate HTML content to display the values
                let mut html_content = String::new();
                html_content.push_str(&data);
                // for (key, value) in values {
                //     html_content.push_str(&format!("{}: {}<br>", key, value));
                // }

                Html(html_content)
            } else {
                Html(format!(
                    "Failed to fetch values. Response status: {}",
                    response.status()
                ))
            }
        }
        Err(err) => Html(format!("Failed to fetch values. Error: {}", err)),
    }
}

fn parse_values_from_response(response: &str) -> Vec<(String, String)> {
    let json: serde_json::Value = serde_json::from_str(response).unwrap();

    let gross_margin = json["GrossMargin"].as_str().unwrap_or("N/A").to_owned();
    let net_margin = json["NetProfitMargin"].as_str().unwrap_or("N/A").to_owned();
    let net_income = json["NetIncome"].as_str().unwrap_or("N/A").to_owned();
    let free_cash_flow = json["FreeCashFlow"].as_str().unwrap_or("N/A").to_owned();
    let retained_earnings = json["RetainedEarnings"].as_str().unwrap_or("N/A").to_owned();
    let total_equity = json["TotalEquity"].as_str().unwrap_or("N/A").to_owned();
    let capital_expenditures = json["CapitalExpenditures"].as_str().unwrap_or("N/A").to_owned();
    let dividends_paid = json["DividendsPaid"].as_str().unwrap_or("N/A").to_owned();
    let cash_from_equity = json["CashFromEquity"].as_str().unwrap_or("N/A").to_owned();
    let pe_ratio = json["PERatio"].as_str().unwrap_or("N/A").to_owned();
    let peg = json["PEGRatio"].as_str().unwrap_or("N/A").to_owned();

    vec![
        ("Gross Margin".to_owned(), gross_margin),
        ("Net Margin".to_owned(), net_margin),
        ("Net Income".to_owned(), net_income),
        ("Free Cash Flow".to_owned(), free_cash_flow)]
}