use battery_usage_plan::BatteryUsagePlan;
use chrono::{FixedOffset, Timelike};
use energy_consumption::{EnergyConsumption, EnergyConsumptionForecast};
use optimization_plan::OptimizationPlan;
use electricity_prices::ElectricityPrices;
use serde::de;
use std::{collections::HashMap, str::FromStr};

mod battery_usage_plan;
mod energy_consumption;
mod optimization_plan;
mod electricity_prices;

// 90 percent of the current battery capacity
const _BATTERY_ROUND_TRIP_EFFICIENCY: f64 = 0.9;
const AVERAGE_ELECTRICITY_PRICE_PER_EURO: f64 = 0.43;

// all units are in kW* or in kWh
const MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID: f64 = 7850.0;
const BATTERY_MAXIMUM_CAPACITY: f64 = 500.0;
const _BATTERY_MAXIMUM_CHARGE_RATE: f64 = 400.0;

fn main() -> anyhow::Result<()> {
    let mut optimal_electricity_usage_plan = OptimizationPlan::new();

    for hour in 0..=23 {
        let current_hour = hour;
        let current_electricity_price = get_electricity_price_for_an_hour(&hour).unwrap();

         // the battery is at 50% at the start of the day
        let mut current_battery_capacity: f64 = 0.5 * BATTERY_MAXIMUM_CAPACITY;

        let energy_demand_for_current_hour = get_electricity_demand_for_an_hour(current_hour);

        // go over the 15 minutes division for the current energy demand for the hour and apply the optimization rule
        for demand in energy_demand_for_current_hour.into_iter() {
            let energy_demand = demand.consumption_average_power_interval;


            let energy_demand_has_overflow =
                energy_demand > MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID;

            let overflow = MAXIMUM_ENERGY_CONTRACTUAL_LIMIT_FROM_GRID - energy_demand;

            println!("energy demand {} over flow {}", energy_demand, overflow);
            let high_tariff: bool = current_electricity_price > AVERAGE_ELECTRICITY_PRICE_PER_EURO;

            let battery_is_full = current_battery_capacity == BATTERY_MAXIMUM_CAPACITY;

            if energy_demand_has_overflow && overflow > current_battery_capacity {
                // eprintln!("Error: Battery cannot handle the current load overflow.");
                std::process::exit(1);
            } else if energy_demand_has_overflow && overflow < current_battery_capacity {
                let time_spent_to_handle_the_excess_in_hrs = BATTERY_MAXIMUM_CAPACITY / overflow;

                let battery_usage_duration_in_secs = time_spent_to_handle_the_excess_in_hrs as u64; //TODO: use standard conversion;
                current_battery_capacity -= overflow;

                let battery_usage_plan = BatteryUsagePlan::new(
                    Some(overflow),
                    None,
                    &demand.start,
                    battery_usage_duration_in_secs,
                );

                optimal_electricity_usage_plan.extend_plan_with(battery_usage_plan);
                // run the load on the battery and calculate time needed
            } else if !energy_demand_has_overflow && high_tariff {
                // eprintln!("Error: A vry high price can't charge the battery now ");
                // high cost, skipping charging battery
            } else if !high_tariff && !battery_is_full {
                // charge the battery
                let time_spent_to_charge_the_battery_hrs = BATTERY_MAXIMUM_CAPACITY / overflow;

                let battery_usage_duration_in_secs = time_spent_to_charge_the_battery_hrs as u64; //TODO: use standard conversion;
                current_battery_capacity -= overflow;

                let battery_usage_plan = BatteryUsagePlan::new(
                    Some(overflow),
                    None,
                    &demand.start,
                    battery_usage_duration_in_secs,
                );

                optimal_electricity_usage_plan.extend_plan_with(battery_usage_plan);
            }
        }
    }

    println!("{:#?}", optimal_electricity_usage_plan);

    Ok(())
}

/// import the provided json and  parse to structs
pub fn parse_json<T: de::DeserializeOwned>(file_path: &'static str) -> Result<T, anyhow::Error> where
{
    let file_path = std::path::Path::new(file_path);

    let data = std::fs::read_to_string(file_path)?;

    let parsed_data: T = serde_json::from_str(&data)?;

    Ok(parsed_data)
}

/// get electricity price for the current_hour,
fn get_electricity_price_for_an_hour(hour: &u32) -> Option<f64> {
    let electricity_prices = parse_json::<ElectricityPrices>("electricity_prices.json").unwrap();

    let mut electricity_price_per_hour = HashMap::new();

    for price in &electricity_prices.prices {
        electricity_price_per_hour.insert(
            chrono::DateTime::<FixedOffset>::from_str(&price.start)
                .unwrap()
                .hour(),
            price.market_price_per_kwh,
        );
    }

    electricity_price_per_hour.get(hour).copied()
}

fn get_electricity_demand_for_an_hour(current_hour: u32) -> Vec<EnergyConsumption> {
    let energy_demand =
        parse_json::<EnergyConsumptionForecast>("energy_consumption_profile.json").unwrap();

    let energy_consumption_for_this_hour = energy_demand
        .forecasts
        .iter()
        .filter(|profile| {
            chrono::DateTime::<FixedOffset>::from_str(&profile.start)
                .unwrap()
                .hour()
                == current_hour
        })
        .map(|profile| EnergyConsumption {
            consumption_average_power_interval: profile.consumption_average_power_interval
                / 1000.0f64, // energy demand is converted to kw
            start: profile.start.clone(),
            end: profile.end.clone(),
        })
        .collect();
    energy_consumption_for_this_hour
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

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

    // #[test]
    // fn test_energy_demand_for_current_hour_function() {}

    #[test]
    fn test_get_electricity_price_for_an_hour() {
        let prices_per_hour = HashMap::<u32, f64>::from([
            (23, 0.3057),
            (0, 0.28752),
            (1, 0.28469),
            (2, 0.27583),
            (3, 0.2823),
        ]);

        for (hour, price) in prices_per_hour {
            assert_eq!(get_electricity_price_for_an_hour(&hour), Some(price));
        }
    }
}
