use std::fs;
use axum::{
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Deserialize)]
struct StockVec {
    stock: Vec<[String; 2]>,
}

#[tokio::main]
async fn main() {
    let yaml_content = fs::read_to_string("stock.yaml").expect("Failed to read file");

    let stock_data: StockVec = serde_yaml::from_str(&yaml_content).expect("Failed to parse YAML");
    
    for pair in stock_data.stock {
        let first = &pair[0];
        println!("{}", first);
    }

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}