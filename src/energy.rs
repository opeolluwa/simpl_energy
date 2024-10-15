
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
