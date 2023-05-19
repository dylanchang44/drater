
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct Source {
    pub stock: Vec<[String; 2]>,
    pub data: HashMap<String, Vec<String>>,
}

//value map for data from api, result map for index after calculation
#[derive(Debug, Deserialize)]
pub struct DataMap {
    pub value: HashMap<String, f32>,
    pub result: HashMap<String, f32>,
}

#[derive(Debug, Deserialize)]
pub struct Drater {
    pub source:Source,
    pub datamap:DataMap
}

impl Drater{

    pub fn new()->Drater{
        Drater{
            source: Source{
                stock: Vec::new(),
                data: HashMap::new(),
            },
            datamap: DataMap {
                value: HashMap::new(),
                result: HashMap::new(),
            }
        }
    }

    pub async fn fetch_data(&mut self, api_key: &str)->Result<(), Box<dyn std::error::Error>>{
        //will be remove after stock loop complete
        let symbol = "AAPL"; // APPLE company symbol
        // Create a mutable reference to self.source.data
        let data_ref = &mut self.source.data; 

        for category in data_ref.keys().cloned().collect::<Vec<_>>(){
            let client = Client::new();
            let endpoint: String = format!("https://www.alphavantage.co/query?function={}&symbol={}&apikey={}",category,symbol, api_key);

            match client.get(&endpoint).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let fetch_result = response.text().await.unwrap();
                        self.parse_fetched_data(&fetch_result, category);
                    } else {
                        return Err(format!("Unsuccessful response. Status code: {}", response.status()).into());
                    }
                }
                Err(err) => return Err(format!("Failed to fetch values. Error: {}", err).into()),
                //html_content.push_str(format!("Failed to fetch values. Error: {}", err)),
            }
        }
        Ok(())
    }
    
    fn parse_fetched_data(&mut self, json: &str, key: String){
        //grap data
        let json: serde_json::Value = serde_json::from_str(json).unwrap();
        //extract data
        let target_vec: Option<&Value> = if key == "OVERVIEW" {
            Some(&json)
        } else {
            Some(&json["quarterlyReports"][0])
        };

        for item in self.source.data[&key].iter().cloned().collect::<Vec<_>>() {
            self.get_value_as_f32(target_vec.unwrap(),&item)
        }
    }
    
    fn get_value_as_f32(&mut self, json: &serde_json::Value, key: &str){
        self.datamap.value.insert(key.to_owned(), json[key].as_str().unwrap_or("-1").parse().unwrap());
    }

    pub fn convert_data(&mut self){
        let source_map=self.datamap.value.clone();
        self.datamap.result.insert("gross_margin".to_owned(), 100.0 * (source_map["grossProfit"]/source_map["totalRevenue"]));
        self.datamap.result.insert("net_margin".to_owned(), 100.0 * (source_map["netIncome"]/source_map["totalRevenue"]));
        self.datamap.result.insert("retained_earning".to_owned(), 100.0 * (source_map["retainedEarnings"]/source_map["totalShareholderEquity"]));
        self.datamap.result.insert("total_equity".to_owned(), 100.0 * (source_map["totalShareholderEquity"]/source_map["netIncome"]));
        self.datamap.result.insert("capital_expenditure".to_owned(), 100.0 * (source_map["capitalExpenditures"]/source_map["netIncome"]));
        self.datamap.result.insert("dividend_paid".to_owned(), 100.0 * (source_map["dividendPayout"]/source_map["operatingCashflow"]));
        self.datamap.result.insert("cash_finance".to_owned(), 100.0 * (source_map["cashflowFromFinancing"]/source_map["operatingCashflow"]));
        self.datamap.result.insert("PERatio".to_owned(), (source_map["PERatio"]));
        self.datamap.result.insert("PEGRatio".to_owned(), (source_map["PEGRatio"]));
    }
}
