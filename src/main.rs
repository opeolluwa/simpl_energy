use std::fmt::Display;

use serde::{de, Deserialize, Serialize};

// 90 percent of the current battery capacity
const BATTERY_ROUND_TRIP_EFFICIENCY: f64 = 0.9;

// all units are in kW* or in kWh
const MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID: u16 = 785;
const BATTERY_MAXIMUM_CAPACITY: u16 = 500;
const BATTERY_MAXIMUM_CHARGE_RATE: u16 = 400;

fn main() -> anyhow::Result<()> {
    let energy_forecast = parse_json::<EnergyForecast>("energy_consumption_profile.json")?;

    let electricity_prices = parse_json::<ElectricityPrices>("electricity_prices.json")?;

    //  *  energy consumption updates every 15 minutes
    //  * when it does, see if:
    //  * 1. the state of the battery charge
    //  * 2. the latest energy demand
    //  *
    //  * if the energy demand is greater than contractual limit, cut off the grid connection, then run on the battery, provided tha battery is charged
    //  *
    //  * if the energy demand is less, and the battery is not fully charged and the price is favorable, charge the battery
    //  */
    // loop {
    // prices update every 1 hour

    // /**

    println!("{} {}", energy_forecast, electricity_prices);

    // }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct EnergyForecast {
    forecasts: Vec<EnergyConsumptionProfile>,
}

impl Display for EnergyForecast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forecasts:\n")?;

        for profile in <Vec<EnergyConsumptionProfile> as Clone>::clone(&self.forecasts).into_iter()
        {
            println!("{:#?}", profile)
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct EnergyConsumptionProfile {
    start: String,
    end: String,
    consumption_average_power_interval: f64,
}

impl Display for EnergyConsumptionProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start: {}\nend: {}\n consumption_average_power_interval:{}",
            self.start, self.end, self.consumption_average_power_interval
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ElectricityPrices {
    bidding_zone: String,
    prices: Vec<EnergyData>,
}

impl Display for ElectricityPrices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bidding_zone: {}\nprices: ", self.bidding_zone)?;

        for price in <Vec<EnergyData> as Clone>::clone(&self.prices).into_iter() {
            println!("{:#?}\n", price)
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct EnergyData {
    start: String,
    end: String,
    market_price_currency: String,
    market_price_per_kwh: f64,
}

impl Display for EnergyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start: {}\nEnd: {}\nMarket Price: {}\n{:.4} per kWh",
            self.start, self.end, self.market_price_currency, self.market_price_per_kwh
        )
    }
}

/// import the provided json and  parse to text
pub fn parse_json<T: de::DeserializeOwned>(file_path: &'static str) -> Result<T, anyhow::Error> where
{
    let file_path = std::path::Path::new(file_path);

    let data = std::fs::read_to_string(file_path)?;

    let parsed_data: T = serde_json::from_str(&data)?;

    Ok(parsed_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Profile {
        name: String,
        gender: String,
        country: String,
    }
    // test the json_parser

    #[test]
    fn test_json_parser() {
        let parsed_data = parse_json::<Profile>("test.json").ok();

        let test_profile = Profile {
            name: "adeoye adefemi".to_string(),
            gender: "male".to_string(),
            country: "nigeria".to_string(),
        };
        assert_eq!(Some(test_profile), parsed_data);
    }
}
