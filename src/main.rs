use std::{fmt::Display, thread, time::Duration};

use serde::{de, Deserialize, Serialize};

// 90 percent of the current battery capacity
const BATTERY_ROUND_TRIP_EFFICIENCY: f64 = 0.9;

// all units are in kW* or in kWh
const MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID: u32 = 785;
const BATTERY_MAXIMUM_CAPACITY: u32 = 500;
const BATTERY_MAXIMUM_CHARGE_RATE: u32 = 400;

const FIFTEEN_MINUTES: Duration = Duration::from_secs(1 * 1);

fn main() -> anyhow::Result<()> {
    let energy_forecast = parse_json::<EnergyForecast>("energy_consumption_profile.json")?;

    let electricity_prices = parse_json::<ElectricityPrices>("electricity_prices.json")?;

    let mut current_hour = 0u32;
    let mut interval_count = 0u8; // from zero to 4, representing 15 minutes division of an hour˝

    // track the energy prices every hour and update the current hour count
    // energy usage are tracked every 15 minutes, of every hour,
    // see the energy consumption
    // see the battery level
    // see if there is need for a recharge or battery usage
    'energy_price: loop {
        let mut current_price_per_kwh = &electricity_prices.prices[0];

        'energy_consumption: loop {
            if current_hour == 5 && interval_count == 3 {
                break 'energy_consumption;
            } else if interval_count == 3 {
                interval_count = 0;
                current_hour += 1;
                current_price_per_kwh = &electricity_prices.prices[1]
            } else {
                interval_count += 1;
            }
            println!(
                "current hour={}, current_interval={}\n",
                current_hour, interval_count
            );

            optimize_energy_usage(current_price_per_kwh, 5.6, 35);

            thread::sleep(FIFTEEN_MINUTES);
        }
    }
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

// optimize the power
fn optimize_energy_usage(
    current_price_per_kwh: &EnergyData,
    current_average_power_consumption: f64,
    current_battery_capacity: u32,
) {
    // if the current energy demand is beyond the volume the grid is supposed to supply

    if current_average_power_consumption > MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID as f64 {
        println!("consider using battery if not  low")
    }

    if current_average_power_consumption < MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID as f64
        && current_battery_capacity < BATTERY_MAXIMUM_CAPACITY
    {
        println!("consider charging the battery now ")
    }
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
