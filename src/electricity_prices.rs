use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub struct ElectricityPrices {
    pub bidding_zone: String,
    pub prices: Vec<ElectricityPricePerHour>,
}

impl Display for ElectricityPrices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bidding_zone: {}\nprices: ", self.bidding_zone)?;

        for price in <Vec<ElectricityPricePerHour> as Clone>::clone(&self.prices).into_iter() {
            writeln!(f, "{:#?}", price)?
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ElectricityPricePerHour {
    pub start: String,
    pub end: String,
    pub market_price_currency: String,
    pub market_price_per_kwh: f64,
}

impl Display for ElectricityPricePerHour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start: {}\nEnd: {}\nMarket Price: {}\n{:.4} per kWh",
            self.start, self.end, self.market_price_currency, self.market_price_per_kwh
        )
    }
}
