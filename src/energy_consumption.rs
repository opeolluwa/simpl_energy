use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
