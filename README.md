# drater
<h4 align="center">Simple web to calculate Stock Analyst Rating of a company using simple linear algebra, based on rust axum web framework</h4>

## Weight
After a score(out of 100) is calculated for fundamental data of the company.
Mutiply a vector of weight to come out the final buying rate.
### Formula 
buying rate =  
5.0 -  
[gross_margin, net_margin, retained_earning, total_equity, capital_expenditure, dividend_paid, cash_finance, PERatio, PEGRatio]  
*  
[0.15,0.1,0.1,0.15,0.1,0.05,0.1,0.2,0.05]  
*  
4/100  
