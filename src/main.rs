mod fetch;

use axum::{
    routing::get,
    http::{Response, StatusCode},
    response::Html,
    Router,
};
use std::collections::HashMap;
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
    let api_key = "IUWE71WEWZMLHPJK";
    let stock_list = std::fs::read_to_string("stock.yaml").expect("Failed to read file");
    let data_list= std::fs::read_to_string("data.yaml").expect("Failed to read file");

    let mut drater=Drater::new();
    //load to drater-source
    drater.source.stock = serde_yaml::from_str(&stock_list).expect("Failed to parse YAML");
    drater.source.data = serde_yaml::from_str(&data_list).expect("Failed to parse YAML");
    //grap index result from http endpoint
    drater.fetch_data(api_key).await.unwrap();
    //calculate rating
    //let rating:HashMap<String, f32>= HashMap::new();
    //manifest

// Generate HTML content to display the values
let mut html_content = String::new();

//print result map
//let limit=5;
                // for (k, v) in drater.company_data {
                //     html_content.push_str(&format!("{}: {:?}<br>", k, v.value));
                // }

                //html_content.push_str(&format!("hashmap size: {}",drater.company_data.len()));
//print data list
                // for (key, value) in drater.source.data {
                //     html_content.push_str(&format!("{}: {:?}<br>", key, value));
                // }

    Html(html_content)
} 

