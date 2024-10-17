use battery_usage_plan::BatteryUsagePlan;
use chrono::{FixedOffset, Timelike};
use electricity_prices::ElectricityPrices;
use energy_consumption::{EnergyConsumption, EnergyConsumptionForecast};
use optimization_plan::OptimizationPlan;
use serde::de;
use std::{collections::HashMap, str::FromStr};

mod battery_usage_plan;
mod electricity_prices;
mod energy_consumption;
mod optimization_plan;

// 90 percent of the current battery capacity
const _BATTERY_ROUND_TRIP_EFFICIENCY_IN: f64 = 0.9;
const AVERAGE_ELECTRICITY_PRICE_IN_EURO: f64 = 0.43;

// all units are in kW* or in kWh
const ENERGY_CONTRACTUAL_LIMIT_FROM_GRID: f64 = 7850.0;
const BATTERY_CAPACITY: f64 = 500.0;
const BATTERY_CHARGE_RATE: f64 = 400.0;

fn main() -> anyhow::Result<()> {
    let mut optimal_electricity_usage_plan = OptimizationPlan::new();

    for hour in 0..=23 {
        let current_hour = hour;
        let current_electricity_price = get_electricity_price_for_an_hour(&hour).unwrap();

        // the battery is at 50% at the start of the day
        let mut current_battery_capacity: f64 = 0.5 * BATTERY_CAPACITY;

        let energy_demand_for_current_hour = get_energy_demand_for_an_hour(current_hour);

        // go over the 15 minutes division for the current energy demand for the hour and apply the optimization rule
        for demand in energy_demand_for_current_hour.into_iter() {
            let energy_demand = demand.consumption_average_power_interval;

            let overflow = ENERGY_CONTRACTUAL_LIMIT_FROM_GRID - energy_demand;

            let energy_demand_has_overflow = overflow < 0.0; // the overflow is a negative value

            let high_tariff: bool = current_electricity_price > AVERAGE_ELECTRICITY_PRICE_IN_EURO;

            let battery_is_full = current_battery_capacity == BATTERY_CAPACITY;

            if energy_demand_has_overflow && overflow.abs() > current_battery_capacity {
                // eprintln!("Error: Battery cannot handle the current load overflow.");
            }

            if energy_demand_has_overflow && overflow.abs() < current_battery_capacity {
                current_battery_capacity -= overflow.abs();

                let battery_usage_plan =
                    BatteryUsagePlan::new(Some(overflow.abs()), None, &demand.start, &demand.end);

                optimal_electricity_usage_plan.extend_plan_with(battery_usage_plan);
            }

            if !energy_demand_has_overflow && high_tariff {
                // eprintln!("Error: A vry high price can't charge the battery now ");
            }

            if !energy_demand_has_overflow && !high_tariff && !battery_is_full {
                let available_energy = overflow;

                // charge the battery
                // if the available energy is greater than the battery charge rate, 400kw, the battery can only take 400kw
                let energy_drawn = available_energy.min(BATTERY_CHARGE_RATE);

                if battery_is_full {
                    current_battery_capacity = BATTERY_CAPACITY;
                }
                let battery_usage_plan =
                    BatteryUsagePlan::new(None, Some(energy_drawn), &demand.start, &demand.end);

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

fn get_energy_demand_for_an_hour(current_hour: u32) -> Vec<EnergyConsumption> {
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
                / 1000.0f64, // energy demand is converted to kilowatt fro watt
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

    #[test]
    fn test_json_parser_function() {
        let parsed_data = parse_json::<Profile>("json_parser_test.json").ok();

        let test_profile = Profile {
            name: "adeoye adefemi".to_string(),
            gender: "male".to_string(),
            country: "nigeria".to_string(),
        };
        assert_eq!(Some(test_profile), parsed_data);
    }

    #[test]
    fn test_get_energy_demand_for_current_hour_function() {
        let energy_demand_for_23rd_hour = vec![
            EnergyConsumption {
                start: "2022-12-12T23:00:00Z".to_string(),
                end: "2022-12-12T23:15:00Z".to_string(),
                consumption_average_power_interval: 4656.0,
            },
            EnergyConsumption {
                start: "2022-12-12T23:15:00Z".to_string(),
                end: "2022-12-12T23:30:00Z".to_string(),
                consumption_average_power_interval: 4528.0,
            },
            EnergyConsumption {
                start: "2022-12-12T23:30:00Z".to_string(),
                end: "2022-12-12T23:45:00Z".to_string(),
                consumption_average_power_interval: 4464.0,
            },
            EnergyConsumption {
                start: "2022-12-12T23:45:00Z".to_string(),
                end: "2022-12-13T00:00:00Z".to_string(),
                consumption_average_power_interval: 4560.0,
            },
        ];

        for interval_demand in 0..=3 {
            assert_eq!(
                energy_demand_for_23rd_hour[interval_demand],
                get_energy_demand_for_an_hour(23)[interval_demand]
            )
        }
    }

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
