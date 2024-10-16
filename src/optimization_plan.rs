use std::fmt::Display;

use crate::battery_usage_plan::BatteryUsagePlan;

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
            writeln!(f, "{:#?}", plan)?;
        }

        Ok(())
    }
}
