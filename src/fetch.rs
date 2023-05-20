
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use std::error::Error;

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
    //key to store company name, value to store all relevant data
    pub company_data:HashMap<String, DataMap>
}

impl Drater{

    pub fn new()->Drater{
        Drater{
            source: Source{
                stock: Vec::new(),
                data: HashMap::new(),
            },
            company_data:HashMap::new()
        }
    }

    pub async fn fetch_data(&mut self, api_key: &str)->Result<(), Box<dyn std::error::Error>>{
        //loop company
        for company in self.source.stock.clone(){
            let symbol=&company[0].clone();
            // Create a mutable reference to self.source.data
            let data_ref = &mut self.source.data; 

            for category in data_ref.keys().cloned().collect::<Vec<_>>(){
                let client = Client::new();
                print!("{}",*symbol);
                let endpoint: String = format!("https://www.alphavantage.co/query?function={}&symbol={}&apikey={}",category,*symbol, api_key);

            match client.get(&endpoint).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let fetch_result = response.text().await.unwrap();
                        self.parse_fetched_data(symbol.clone(), category, &fetch_result);
                        //self.convert_company_data(symbol.clone()).unwrap();
                    } else {
                        return Err(format!("Unsuccessful response. Status code: {}", response.status()).into());
                    }
                }
                Err(err) => return Err(format!("Failed to fetch values. Error: {}", err).into()),
                //html_content.push_str(format!("Failed to fetch values. Error: {}", err)),
            }
        }
        }
        Ok(()) 
    }
    
    fn parse_fetched_data(&mut self, symbol: String ,keyword: String, json: &str){
        //grap data
        let json: serde_json::Value = serde_json::from_str(json).unwrap();
        //extract data
        let target_vec: Option<&Value> = if keyword == "OVERVIEW" {
            Some(&json)
        } else {
            Some(&json["quarterlyReports"][0])
        };

        for item in self.source.data[&keyword].iter().cloned().collect::<Vec<_>>() {
            let _= self.get_value_for_company(target_vec.unwrap(), &symbol,&item);
        }
    }
    
    fn get_value_for_company(&mut self, json: &serde_json::Value, symbol: &String, keyword: &str)->Result<(),Box<dyn std::error::Error>>{
        //debug
        if let Some(datamap)=self.company_data.get_mut(symbol){
            datamap.value.insert(keyword.to_owned(), json[keyword].as_str().unwrap_or("-1").parse().unwrap());
        }else{
            let error_message = format!("Can't get datamap of company_data at mutable for {}", symbol);
            return Err(Box::<dyn Error>::from(error_message));
        }
        Ok(())
    }

    fn convert_company_data(&mut self, symbol: String)->Result<(),Box<dyn std::error::Error>>{
        let source_map=self.company_data[&symbol].value.clone();

        if let Some(datamap)=self.company_data.get_mut(&symbol){
            datamap.result.insert("gross_margin".to_owned(), 100.0 * (source_map["grossProfit"]/source_map["totalRevenue"]));
            datamap.result.insert("net_margin".to_owned(), 100.0 * (source_map["netIncome"]/source_map["totalRevenue"]));
            datamap.result.insert("retained_earning".to_owned(), 100.0 * (source_map["retainedEarnings"]/source_map["totalShareholderEquity"]));
            datamap.result.insert("total_equity".to_owned(), 100.0 * (source_map["totalShareholderEquity"]/source_map["netIncome"]));
            datamap.result.insert("capital_expenditure".to_owned(), 100.0 * (source_map["capitalExpenditures"]/source_map["netIncome"]));
            datamap.result.insert("dividend_paid".to_owned(), 100.0 * (source_map["dividendPayout"]/source_map["operatingCashflow"]));
            datamap.result.insert("cash_finance".to_owned(), 100.0 * (source_map["cashflowFromFinancing"]/source_map["operatingCashflow"]));
            datamap.result.insert("PERatio".to_owned(), source_map["PERatio"]);
            datamap.result.insert("PEGRatio".to_owned(), source_map["PEGRatio"]);
        }else{
            let error_message = format!("Can't get datamap of company_data at mutable for {}", symbol);
            return Err(Box::<dyn Error>::from(error_message));
        }
        Ok(())
    }
}
