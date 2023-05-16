use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

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

#[tokio::main]
async fn main() {
    let yaml_content = std::fs::read_to_string("stock.yaml").expect("Failed to read file");
    let stock_data: StockVec = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");
    
    // Set up the Axum web server
    let app = Router::new().route("/", get(launch));

    // Start the web server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct StockVec {
    stock: Vec<[String; 2]>,
}

//value map for data from api, result map for index after calculation
#[derive(Debug, Deserialize)]
struct DataMap {
    value: HashMap<String, f32>,
    result: HashMap<String, f32>,
}

async fn launch() -> Html<String> {
    // //for extract api page
    // let api_key = "IUWE71WEWZMLHPJK";
    // let symbol = "AAPL"; // APPLE company symbol
    // let client = Client::new();
    // let endpoint: String = format!(
    //     "https://www.alphavantage.co/query?function=CASH_FLOW&symbol={}&apikey={}",
    //     symbol, api_key
    // );

    // match client.get(&endpoint).send().await {
    //     Ok(response) => {
    //         if response.status().is_success() {
    //             let data = response.text().await.unwrap();
    //             //let values = parse_values_from_response(&data);

    //             // Generate HTML content to display the values
    //             let mut html_content = String::new();
    //             html_content.push_str(&data);
    //             // for (key, value) in values {
    //             //     html_content.push_str(&format!("{}: {}<br>", key, value));
    //             // }

    //             Html(html_content)
    //         } else {
    //             Html(format!(
    //                 "Failed to fetch values. Response status: {}",
    //                 response.status()
    //             ))
    //         }
    //     }
    //     Err(err) => Html(format!("Failed to fetch values. Error: {}", err)),
    // }
    let api_key = "IUWE71WEWZMLHPJK";
    let symbol = "AAPL"; // APPLE company symbol

    let mut data= DataMap {
        value: HashMap::new(),
        result: HashMap::new(),
    };

    data.fetch_data(api_key,symbol);

// Generate HTML content to display the values
    let mut html_content = String::new();
                //html_content.push_str(&data);
                for (key, value) in data.result {
                    html_content.push_str(&format!("{}: {}<br>", key, value));
                }
    Html(html_content)
}

impl DataMap{

    async fn fetch_data(&mut self, api_key: &str, symbol: &str){
        //  get INCOME_STATEMENT data
        self.fetch_income_data(api_key,symbol).await;
        //  get BALANCE_SHEET data
        self.fetch_balance_data(api_key,symbol).await;
    }

    async fn fetch_income_data(&mut self, api_key: &str, symbol: &str){
        let client = Client::new();
        let endpoint: String = format!("https://www.alphavantage.co/query?function=INCOME_STATEMENT&symbol={}&apikey={}",symbol, api_key);
        //http request
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let data = response.text().await.unwrap();
                    self.parse_values_from_income(&data);
    
                } else {
                    
                }
            }
            Err(err) => println!("fail"),
            //html_content.push_str(format!("Failed to fetch values. Error: {}", err)),
        }
    }
    
    async fn fetch_balance_data(&mut self, api_key: &str, symbol: &str){
        let client = Client::new();
        let endpoint: String = format!("https://www.alphavantage.co/query?function=BALANCE_SHEET&symbol={}&apikey={}",symbol, api_key);
        //http request
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let data = response.text().await.unwrap();
                    self.parse_values_from_income(&data);
    
                } else {
                    
                }
            }
            Err(err) => println!("fail"),
        }
    }

    async fn fetch_cash_data(&mut self, api_key: &str, symbol: &str){
        let client = Client::new();
        let endpoint: String = format!("https://www.alphavantage.co/query?function=CASH_FLOW&symbol={}&apikey={}",symbol, api_key);
        //http request
        match client.get(&endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let data = response.text().await.unwrap();
                    self.parse_values_from_income(&data);
    
                } else {
                    
                }
            }
            Err(err) => println!("fail"),
            //html_content.push_str(format!("Failed to fetch values. Error: {}", err)),
        }
    }
    
    fn parse_values_from_income(&mut self, json: &str){
        //grap data
        let json: serde_json::Value = serde_json::from_str(json).unwrap();
        //extract quarterly data
        let q_data: &Value=&json["quarterlyReports"][0];
    
        //let revenue=get_value_as_f32(q_data,"totalRevenue");
        //gross_margin
        //let gross_profit= get_value_as_f32(q_data,"grossProfit");
        //let gross_margin=100.0 * (gross_profit/revenue);
        //net_margin
        //let net_income = get_value_as_f32(q_data,"netIncome");
        //let net_margin=100.0 * (net_income/revenue);
        //
        let total_equity = json["TotalEquity"].as_str().unwrap_or("N/A").to_owned();
        let capital_expenditures = json["CapitalExpenditures"].as_str().unwrap_or("N/A").to_owned();
        let dividends_paid = json["DividendsPaid"].as_str().unwrap_or("N/A").to_owned();
        let cash_from_equity = json["CashFromEquity"].as_str().unwrap_or("N/A").to_owned();
        let pe_ratio = json["PERatio"].as_str().unwrap_or("N/A").to_owned();
        let peg = json["PEGRatio"].as_str().unwrap_or("N/A").to_owned();
    }
    
    fn parse_values_from_balance(response: &str, map: &mut HashMap<String, f32>){
        //grap data
        let json: serde_json::Value = serde_json::from_str(response).unwrap();
        //extract quarterly data
        let q_data: &Value=&json["quarterlyReports"][0];
        //required balance data
        let v=vec!["totalShareholderEquity","retainedEarnings"];
        //net_margin

        //
        let total_equity = json["TotalEquity"].as_str().unwrap_or("N/A").to_owned();
        let capital_expenditures = json["CapitalExpenditures"].as_str().unwrap_or("N/A").to_owned();
        let dividends_paid = json["DividendsPaid"].as_str().unwrap_or("N/A").to_owned();
        let cash_from_equity = json["CashFromEquity"].as_str().unwrap_or("N/A").to_owned();
        let pe_ratio = json["PERatio"].as_str().unwrap_or("N/A").to_owned();
        let peg = json["PEGRatio"].as_str().unwrap_or("N/A").to_owned();
    }
    //let retained_earnings =
    
    fn get_value_as_f32(data: &serde_json::Value, key: &str) -> f32 {
        data[key].as_str().unwrap_or("-1").parse().unwrap()
    }
}
