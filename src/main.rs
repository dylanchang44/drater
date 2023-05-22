mod fetch;
use std::fs;
use reqwest::Client;
use axum::{
    routing::get,
    response::Html,
    Router,
};
use serde_yaml::{self};
use fetch::Drater;

#[tokio::main]
async fn main() {
    // Set up the Axum web server
    let app = Router::new().route("/", get(launch));

    // Start the web server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn launch() -> Html<String> {
    //my standard API key of Alpha Vantage
    let api_key = "IUWE71WEWZMLHPJK";
    //let stock_list = std::fs::read_to_string("stock.yaml").expect("Failed to read file");
    let data_list= std::fs::read_to_string("data.yaml").expect("Failed to read file");

    let mut drater=Drater::new();
    //load to drater-source
    drater.source.data = serde_yaml::from_str(&data_list).expect("Failed to parse YAML");
    //grap index result from http endpoint
    let symbol="PYPL";
    drater.fetch_data(api_key,symbol).await.unwrap();
    let rating=drater.rating_calc();

// Generate HTML content to display the values
let mut html_content = String::new();
html_content.push_str(&format!("{} rating: {:.2}<br>", symbol, rating));

fs::write("./index.html", &html_content).expect("Failed to write index.html");

//Test API
/* 
let endpoint: String = format!("https://www.alphavantage.co/query?function={}&symbol={}&apikey={}","CASH_FLOW","TSLA", api_key);
    let client = Client::new();
    match client.get(&endpoint).send().await {
        Ok(response) => {
            if response.status().is_success() {
                let fetch_result = response.text().await.unwrap();
                html_content.push_str(&fetch_result);
                //self.convert_company_data(symbol.clone()).unwrap();
            } else {
                println!("{}", format!("Unsuccessful response. Status code: {}", response.status()));
            }
        }
        Err(err) => println!("{}", format!("Failed to fetch values. Error: {}", err))
    }
*/

//print result map
                // for (k, v) in drater.company_data.value {
                //     html_content.push_str(&format!("{}: {}<br>", k, v));
                // }

                //html_content.push_str(&format!("hashmap size: {}",drater.company_data.len()));
//print data list
                // for (k, v) in drater.company_data.normalized_result {
                //     html_content.push_str(&format!("{}: {:?}<br>", k, v));
                // }

    Html(html_content)
} 

