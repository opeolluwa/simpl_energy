use std::{fmt::Display, time::Duration};

use chrono::{DateTime, FixedOffset};

use std::str::FromStr;

/// the BatteryUsagePlan states how much energy should be drawn from the battery
#[derive(Debug, Clone)]
pub struct BatteryUsagePlan {
    /// the time (in ISO format) when usage or recharge starts
    pub start: DateTime<FixedOffset>,
    /// the time (in ISO format) when usage or recharge ends
    pub end: DateTime<FixedOffset>,
    pub energy_from_battery_wh: Option<f64>,
    pub energy_to_battery_wh: Option<f64>,
}

impl BatteryUsagePlan {
    pub fn new(
        energy_drawn_from_battery: Option<f64>,
        energy_fed_into_battery: Option<f64>,
        start: &str,
        duration: u64, // the time taken in minutes
    ) -> Self {
        let start = DateTime::<FixedOffset>::from_str(start).unwrap();
        let end = start + Duration::from_secs(duration);
        Self {
            start,
            end, // the start  time+ duration
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
