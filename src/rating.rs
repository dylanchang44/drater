use std::collections::BTreeMap;

//store first 3 and last 3 ranking stock symbol
//binary heap or dequeue?
pub struct Rank{
    pub top:BTreeMap<Stock>,
    pub bottom:BTreeMap<Stock>
}