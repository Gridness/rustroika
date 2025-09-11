use crate::types::*;
use std::cmp::Ordering;

pub fn calculate_transport_costs(config: CalculationConfig) -> CalculationResult {
    let total_trips = config.trips_per_week * 4;

    let individual_cost = if config.discount_enabled {
        calculate_with_discount(total_trips, config.ticket_price)
    } else {
        total_trips * config.ticket_price
    };

    let (recommendation, savings) = match individual_cost.cmp(&config.monthly_cost) {
        Ordering::Less => (Recommendation::Individual, config.monthly_cost - individual_cost),
        Ordering::Equal => (Recommendation::Equal, 0),
        Ordering::Greater => (Recommendation::MonthlyPass, individual_cost - config.monthly_cost),
    };

    CalculationResult { total_trips, monthly_cost: config.monthly_cost, individual_cost, ticket_price: config.ticket_price, recommendation, savings  }
}

fn calculate_with_discount(total_trips: u32, ticket_price: u32) -> u32 {
    let full_price_count = total_trips.div_ceil(2);
    let discounted_count = total_trips / 2;

    full_price_count * ticket_price + discounted_count * (ticket_price / 2)
}

#[cfg(feature = "detailed-output")]
fn calculate_breakdown(total_trips: u32, ticket_price: u32) -> CostBreakdown {
    let full_price_trips = total_trips.div_ceil(2);
    let discounted_trips = total_trips / 2;
    let full_price_cost = full_price_trips * ticket_price;
    let discounted_cost = discounted_trips * (ticket_price / 2);
    let discounted_savings = discounted_trips * (ticket_price - ticket_price / 2);

    CostBreakdown {
        full_price_trips,
        discounted_trips,
        full_price_cost,
        discounted_cost,
        discounted_savings,
    }
}

#[cfg(feature = "annual-pass")]
pub fn calculate_annual_pass(monthly_cost: u32, annual_multiplier: f32) -> AnnualPassCalculation {
    let annual_cost = (monthly_cost as f32 * annual_multiplier) as u32;
    let monthly_equivalent = annual_cost / 12;
    let annual_savings = (monthly_cost * 12).saturating_sub(annual_cost);

    AnnualPassCalculation {
        annual_cost,
        monthly_equivalent,
        annual_savings,
    }
}

pub fn optimize_trip_distribution(weekly_trips: u32) -> (u32, u32) {
    let total_monthly_trips = weekly_trips * 4;
    let full_price_trips = total_monthly_trips.div_ceil(2);
    let discounted_trips = total_monthly_trips / 2;

    (full_price_trips, discounted_trips)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_discount_calculation() {
        let result = calculate_with_discount(10, 63);
        assert_eq!(result, 470);
    }

    #[test]
    fn test_monthly_pass_recommendation() {
        let config = CalculationConfig { trips_per_week: 20, monthly_cost: 2900, ticket_price: 63, discount_enabled: true};

        let result = calculate_transport_costs(config);
        assert_eq!(result.recommendation, Recommendation::MonthlyPass);
        assert!(result.savings > 0);
    }

    #[test]
    fn test_individual_recommendation() {
        let config = CalculationConfig { trips_per_week: 5, monthly_cost: 2900, ticket_price: 63, discount_enabled: true};

        let result = calculate_transport_costs(config);
        assert_eq!(result.recommendation, Recommendation::Individual);
        assert!(result.savings > 0);
    }

    #[test]
    fn test_no_discount() {
        let config = CalculationConfig { trips_per_week: 10, monthly_cost: 2900, ticket_price: 63, discount_enabled: false};

        let result = calculate_transport_costs(config);
        assert_eq!(result.individual_cost, 10 * 4 * 63);
    }

    #[test]
    fn test_trip_optimization() {
        let (full, discounted) = optimize_trip_distribution(10);
        assert_eq!(full, 20);
        assert_eq!(discounted, 20);
    }
}
