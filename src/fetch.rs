
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, clone};
use reqwest::Client;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct Source {
    pub data: HashMap<String, Vec<String>>,
}

//value map for data from api, result map for index after calculation
#[derive(Debug, Deserialize)]
pub struct DataMap {
    pub value: HashMap<String, f32>,
    pub result: HashMap<String, f32>,
    pub normalized_result: HashMap<String, f32>
}

#[derive(Debug, Deserialize)]
pub struct Drater {
    pub source:Source,
    //key to store company name, value to store all relevant data
    pub company_data:DataMap
}

impl Drater{

    pub fn new()->Drater{
        Drater{
            source: Source{
                data: HashMap::new(),
            },
            company_data:DataMap{
                value: HashMap::new(),
                result: HashMap::new(),
                normalized_result: HashMap::new(),
            },
        }
    }

    pub async fn fetch_data(&mut self,api_key: &str, symbol:&str)->Result<(), Box<dyn std::error::Error>>{
            for category in self.source.data.keys().cloned().collect::<Vec<_>>(){
                let client = Client::new();
                let endpoint: String = format!("https://www.alphavantage.co/query?function={}&symbol={}&apikey={}",category,symbol, api_key);

            match client.get(&endpoint).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let fetch_result = response.text().await.unwrap();
                        self.parse_fetched_data(category, &fetch_result);
                    } else {
                        return Err(format!("Unsuccessful response. Status code: {}", response.status()).into());
                    }
                }
                Err(err) => return Err(format!("Failed to fetch values. Error: {}", err).into()),
            }
        }
        self.convert_company_data();
        self.normalize_company_data();
        Ok(()) 
    }
    
    fn parse_fetched_data(&mut self, keyword: String, json: &str){
        // Parse the JSON data
        let json_text: serde_json::Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            return; // or handle the error in an appropriate way
            }
        };

        //extract data vector
        let target_vec = if keyword == "OVERVIEW" {
            json_text
        } else {
            //json_text["quarterlyReports"].as_array().and_then(|arr| arr.get(0))
            json_text["quarterlyReports"][0].clone()
        };
        for item in self.source.data[&keyword].clone() {
             self.company_data.value.insert(item.clone(), target_vec[item.clone()].as_str().and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0));
        }
    }

    fn convert_company_data(&mut self){
        let source_map=self.company_data.value.clone();
        let result=&mut self.company_data.result;
            result.insert("gross_margin".to_owned(), 100.0 * (source_map["grossProfit"]/source_map["totalRevenue"].abs()));
            result.insert("net_margin".to_owned(), 100.0 * (source_map["netIncome"]/source_map["totalRevenue"].abs()));
            result.insert("retained_earning".to_owned(), 100.0 * (source_map["retainedEarnings"]/source_map["totalShareholderEquity"].abs()));
            result.insert("total_equity".to_owned(), 100.0 * (source_map["totalShareholderEquity"]/source_map["netIncome"].abs()));
            result.insert("capital_expenditure".to_owned(), 100.0 * (source_map["capitalExpenditures"]/source_map["netIncome"].abs()));
            result.insert("dividend_paid".to_owned(), 100.0 * (source_map["dividendPayout"]/source_map["operatingCashflow"].abs()));
            result.insert("cash_finance".to_owned(), 100.0 * (source_map["cashflowFromFinancing"]/source_map["operatingCashflow"].abs()));
            result.insert("PERatio".to_owned(), source_map["PERatio"]);
            result.insert("PEGRatio".to_owned(), source_map["PEGRatio"]);
    }

    fn normalize_company_data(&mut self){
        let source_map: HashMap<String, f32>=self.company_data.result.clone();
        let result=&mut self.company_data.normalized_result;
            result.insert("gross_margin".to_owned(), source_map["gross_margin"]);  //0~100
            result.insert("net_margin".to_owned(), 5.0 * source_map["net_margin"]); //0~20
            result.insert("retained_earning".to_owned(),(source_map["retained_earning"] + 200.0)/10.0);  //-200~800
            result.insert("total_equity".to_owned(), source_map["total_equity"]/30.0 ); // 0~3000
            result.insert("capital_expenditure".to_owned(), source_map["capital_expenditure"]); // 0~100
            result.insert("dividend_paid".to_owned(), 2.0 * source_map["dividend_paid"]); //0~50
            result.insert("cash_finance".to_owned(), (source_map["cash_finance"] + 200.0)/4.0);  //-200~200
            result.insert("PERatio".to_owned(), (120.0 - source_map["PERatio"]) * 5.0/6.0);    //0~120
            result.insert("PEGRatio".to_owned(), 100.0 - source_map["PEGRatio"] * 100.0/3.0);  // 0~3
    }

    pub fn rating_calc(self)->f32{
        let n_map=
        [
        self.company_data.normalized_result["gross_margin"],
        self.company_data.normalized_result["net_margin"],
        self.company_data.normalized_result["retained_earning"],
        self.company_data.normalized_result["total_equity"],
        self.company_data.normalized_result["capital_expenditure"],
        self.company_data.normalized_result["dividend_paid"],
        self.company_data.normalized_result["cash_finance"],
        self.company_data.normalized_result["PERatio"],
        self.company_data.normalized_result["PEGRatio"]
        ];

        let weight_vec=[0.15,0.1,0.1,0.15,0.1,0.05,0.1,0.2,0.05];
        let mut rating:f32 = 0.0;
        for it in 0..9{
            rating += n_map[it]*weight_vec[it];
        }
        rating=1.0+rating/25.0;
        rating
    }
}
