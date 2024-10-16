use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{fmt::Display, time::Duration};

use crate::battery_usage_plan::BatteryUsagePlan;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnergyConsumptionForecast {
    pub forecasts: Vec<EnergyConsumption>,
}

impl Display for EnergyConsumptionForecast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forecasts:")?;

        for profile in <Vec<EnergyConsumption> as Clone>::clone(&self.forecasts).into_iter() {
            writeln!(f, "{:#?}", profile)?
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnergyConsumption {
    pub start: String,
    pub end: String,
    pub consumption_average_power_interval: f64,
}

impl Display for EnergyConsumption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "start: {}\nend: {}\n consumption_average_power_interval:{}",
            self.start, self.end, self.consumption_average_power_interval
        )
    }
}
/// store the battery usage plan  
#[derive(Debug)]
pub struct OptimizationPlan {
    planning: Vec<BatteryUsagePlan>,
}

impl OptimizationPlan {
    pub fn new() -> Self {
        Self {
            planning: Vec::<BatteryUsagePlan>::new(),
        }
    }
    pub fn extend_plan_with(&mut self, plan: BatteryUsagePlan) {
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
