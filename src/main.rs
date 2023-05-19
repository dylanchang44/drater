use axum::{
    routing::get,
    http::{Response, StatusCode},
    response::Html,
    Router,
};

use serde_yaml::{self};

mod fetch;
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
    let stock_list = std::fs::read_to_string("stock.yaml").expect("Failed to read file");
    let data_list= std::fs::read_to_string("data.yaml").expect("Failed to read file");
    let mut drater=Drater::new();

    //load to drater-source
    drater.source.stock = serde_yaml::from_str(&stock_list).expect("Failed to parse YAML");
    drater.source.data = serde_yaml::from_str(&data_list).expect("Failed to parse YAML");

    let api_key = "IUWE71WEWZMLHPJK";

    let _ = drater.fetch_data(api_key).await;
    drater.convert_data();

// Generate HTML content to display the values
let mut html_content = String::new();

//print result map
                for (key, value) in drater.datamap.result {
                    html_content.push_str(&format!("{}: {}<br>", key, value));
                }
//print data list
                // for (key, value) in drater.source.data {
                //     html_content.push_str(&format!("{}: {:?}<br>", key, value));
                // }
//print stock list
                // for ele in list.stock {
                //     html_content.push_str(&format!("{}: {}<br>", ele[0], ele[1]));
                // }
    Html(html_content)
} 