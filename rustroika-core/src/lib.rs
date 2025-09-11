pub mod calculation;
pub mod types;

pub use calculation::*;
pub use types::*;

pub fn calculate_best_option(trips_per_week: u32, monthly_cost: u32, ticket_price: u32) -> CalculationResult {
    let config = CalculationConfig { trips_per_week, monthly_cost, ticket_price, discount_enabled: cfg!(feature = "discount-calculation") };

    calculate_transport_costs(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_calculation() {
        let result = calculate_best_option(10, 2900, 63);
        assert!(result.total_trips > 0);
        assert!(result.individual_cost > 0);
    }

    #[test]
    fn test_monthly_pass_better() {
        let result = calculate_best_option(20, 2900, 63);
        assert_eq!(result.recommendation, Recommendation::MonthlyPass);
    }

    #[test]
    fn test_individual_better() {
        let result = calculate_best_option(5, 2900, 63);
        assert_eq!(result.recommendation, Recommendation::Individual);
    }
}
