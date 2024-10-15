use std::{collections::HashMap, fmt::Display, str::FromStr};

use chrono::{DateTime, FixedOffset, Timelike};
use serde::{de, Deserialize, Serialize};

// 90 percent of the current battery capacity
const BATTERY_ROUND_TRIP_EFFICIENCY: f64 = 0.9;
const AVERAGE_ELECTRICITY_PRICE_PER_EURO: f64 = 0.43;

// all units are in kW* or in kWh
const MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID: f64 = 7850.0;
const BATTERY_MAXIMUM_CAPACITY: f64 = 500.0;
const BATTERY_MAXIMUM_CHARGE_RATE: f64 = 400.0;

fn main() -> anyhow::Result<()> {
    let energy_consumption_profile =
        parse_json::<EnergyConsumptionForecast>("energy_consumption_profile.json")?;

    let electricity_prices = parse_json::<ElectricityPrices>("electricity_prices.json")?;

    let mut electricity_price_per_hour = HashMap::new();

    for price in &electricity_prices.prices {
        electricity_price_per_hour.insert(
            chrono::DateTime::<FixedOffset>::from_str(&price.end)
                .unwrap()
                .hour(),
            price.market_price_per_kwh,
        );
    }

    let optimal_electricity_usage_plan = OptimizationPlan::new();

    // go over the energy consumption profile and tell the energy demand
    // the electricity demand changes/is updated every 15 minutes, that's 4 times in an hour
    // the electricity prices is updated every hour
    for profile in energy_consumption_profile.forecasts.into_iter() {
        let current_hour: u32 = chrono::DateTime::<FixedOffset>::from_str(&profile.end)?.hour();

        let energy_demand = profile.consumption_average_power_interval / 1000.0f64; // convert to kilowatt

        let current_battery_capacity: f64 = 0.5 * BATTERY_MAXIMUM_CAPACITY; // the battery is at 505 at the start of the day

        // if the energy demand is greater, just use the battery, don't bother to check the prices, check if the battery is currently charged
        if energy_demand > MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID {
            // check the battery capacity
            // get the remainder left,
            // get the duration the battery will be used to run the load
            let overflow = energy_demand - MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID;

            // fail safe, abort if the battery cannot run the load
            if overflow > current_battery_capacity {
                // TODO: handle the edge case here
                std::process::exit(1);
            } else {
                let power_drawn_from_battery = overflow;
                let time_taken_to_manage_excess =
                    BATTERY_MAXIMUM_CAPACITY / power_drawn_from_battery;

                //todo: factor in the 15minutes constraint
                // println!(
                //     "time taken {} power drawn {}",
                //     time_taken_to_manage_excess, power_drawn_from_battery
                // );

                //TODO: optimal_electricity_usage_plan.extend_plan_with();
            }
            // println!(" the overflow is {}", overflow);
        } else {
            // get the current price, be sure it is lower or equal to the average subscription rate, charged the battery and track how much kw is fed into it
            let current_electricity_price = electricity_price_per_hour.get(&current_hour).unwrap();

            let cost_is_high: bool =
                current_electricity_price > &AVERAGE_ELECTRICITY_PRICE_PER_EURO;

            let battery_is_full = current_battery_capacity == BATTERY_MAXIMUM_CAPACITY;

            if cost_is_high {
                // println!("the cost is too high ")
            } else if battery_is_full {
                // println!("battery is full")
            } else if cost_is_high && !battery_is_full {
                // println!("a very high cost is detected and battery isn't full")
            } else {
                // battery is not full and cost is not high
                // println!("consider charging the battery");
                //TODO: optimal_electricity_usage_plan.extend_plan_with();
            }
        }
    }

    println!("{:?}", optimal_electricity_usage_plan);
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct EnergyConsumptionForecast {
    forecasts: Vec<EnergyConsumptionProfile>,
}

impl Display for EnergyConsumptionForecast {
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
/// the BatteryUsagePlan states how much energy should be drawn from the battery
#[derive(Debug, Clone)]
pub struct BatteryUsagePlan {
    /// the time (in ISO format) when usage or recharge starts
    start: DateTime<FixedOffset>,
    /// the time (in ISO format) when usage or recharge ends
    end: DateTime<FixedOffset>,
    energy_from_battery_wh: Option<f64>,
    energy_to_battery_wh: Option<f64>,
}

impl BatteryUsagePlan {
    fn new(
        energy_drawn_from_battery: Option<f64>,
        energy_fed_into_battery: Option<f64>,
        // start: &DateTime<>,
        duration: u32, // the time taken in minutes
    ) -> Self {
        Self {
            start: todo!(),
            end: todo!(), // the start  time+ duration
            energy_from_battery_wh: energy_drawn_from_battery,
            energy_to_battery_wh: energy_fed_into_battery,
        }
    }
}

impl Display for BatteryUsagePlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start:{:#?},\nend:{:#?},\nenergy_from_battery_wh:{:#?},energy_to_battery_wh:{:#?}",
            self.start, self.end, self.energy_from_battery_wh, self.energy_to_battery_wh
        )
    }
}

/// store the battery usage plan  
#[derive(Debug)]
struct OptimizationPlan {
    planning: Vec<BatteryUsagePlan>,
}

impl OptimizationPlan {
    fn new() -> Self {
        Self {
            planning: Vec::<BatteryUsagePlan>::new(),
        }
    }
    fn extend_plan_with(&mut self, plan: BatteryUsagePlan) {
        self.planning.push(plan);
    }
}

impl Display for OptimizationPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "planning:",)?;
        for plan in <Vec<BatteryUsagePlan> as Clone>::clone(&self.planning).into_iter() {
            println!("{:#?}", plan)
        }

        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize)]
struct ElectricityPrices {
    bidding_zone: String,
    prices: Vec<ElectricityPricePerHour>,
}

impl Display for ElectricityPrices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bidding_zone: {}\nprices: ", self.bidding_zone)?;

        for price in <Vec<ElectricityPricePerHour> as Clone>::clone(&self.prices).into_iter() {
            println!("{:#?}\n", price)
        }
        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ElectricityPricePerHour {
    start: String,
    end: String,
    market_price_currency: String,
    market_price_per_kwh: f64,
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

/// import the provided json and  parse to structs
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
