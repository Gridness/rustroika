use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationConfig {
    pub trips_per_week: u32,
    pub monthly_cost: u32,
    pub ticket_price: u32,
    pub discount_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub total_trips: u32,
    pub monthly_cost: u32,
    pub individual_cost: u32,
    pub ticket_price: u32,
    pub recommendation: Recommendation,
    pub savings: u32,
    #[cfg(feature = "detailed-output")]
    pub breakdown: CostBreakdown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Recommendation {
    MonthlyPass,
    Individual,
    Equal,
}

#[cfg(feature = "detailed-output")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub full_price_trips: u32,
    pub discounted_trips: u32,
    pub full_price_cost: u32,
    pub discounted_cost: u32,
    pub discount_savings: u32,
}

impl CalculationResult {
    pub fn format_recommendation(&self) -> String {
        match self.recommendation {
            Recommendation::MonthlyPass => format!("💰 Monthly pass saves you {} RUB!", self.savings),
            Recommendation::Individual => format!("🚌 Paying per trip is cheaper by {} RUB!", self.savings),
            Recommendation::Equal => "⚖️ Both options cost the same".to_string(),
        }
    }

    pub fn format_recommendation_simple(&self) -> &'static str {
        match self.recommendation {
            Recommendation::MonthlyPass => "Monthly Pass",
            Recommendation::Individual => "Individual",
            Recommendation::Equal => "Equal Cost",
        }
    }
}

#[cfg(feature = "annual-pass")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualPassCalculation {
    pub annual_cost: u32,
    pub monthly_equivalent: u32,
    pub annual_savings: u32,
}
