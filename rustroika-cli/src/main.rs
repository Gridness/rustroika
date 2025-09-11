use rustroika_core::{CalculationConfig, CalculationResult, calculate_transport_costs};
use clap::Parser;
use colored::*;
use std::error::Error;

#[derive(Parser)]
#[command(
    name = "rustroika",
    author = "Ivan King",
    version = "3.0.0",
    about = "A tool for calculating what way of paying for your public transport with Troika card in Moscow is more profitable",
    long_about = "Calculate whether buying a monthly transport pass or paying per trip is more economical for Moscow public transport",
)]
struct Args {
    /// Number of trips per week
    #[arg(short = 't', long = "trips-week", value_name = "TRIPS", help = "Number of trips you make per week")]
    trips_per_week: u32,

    /// Monthly pass cost in RUB
    #[arg(short = 'm', long = "monthly-cost", value_name = "COST", help = "Cost of the monthly pass in RUB")]
    monthly_cost: u32,

    /// Price of a single ticket in RUB
    #[arg(short = 'p', long = "ticket-price", value_name = "PRICE", help = "Cost of a single ticket in RUB")]
    ticket_price: u32,

    /// Disable discount calculations
    #[arg(long = "no-discount", help = "Disable the 50% discount for subsequent trips")]
    no_discount: bool,

    /// Show detailed costs breakdown
    #[cfg(feature = "detailed-output")]
    #[arg(long = "detailed", help = "Show detailed cost breakdown")]
    detailed: bool,

    /// Use plain text output without colors
    #[cfg(feature = "colored-output")]
    #[arg(long = "no-color", help = "Disable colored output")]
    no_color: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.trips_per_week == 0 {
        eprintln!("Error: Number of trips per week must be greater than 0");
        std::process::exit(1);
    }

    let config = CalculationConfig {
        trips_per_week: args.trips_per_week,
        monthly_cost: args.monthly_cost,
        ticket_price: args.ticket_price,
        discount_enabled: !args.no_discount,
    };

    let result = calculate_transport_costs(config);

    #[cfg(feature = "colored-output")]
    {
        if args.no_color {
            display_plain_results(&result, &args);
        } else {
            display_colored_results(&result, &args);
        }
    }

    #[cfg(not(feature = "colored-output"))]
    {
        display_plain_results(&result, &args);
    }

    Ok(())
}

#[cfg(feature = "colored-output")]
fn display_colored_results(result: &CalculationResult, _args: &Args) {
    let col1_data_plain = result.total_trips.to_string();
    let col2_data_plain = format!("{} RUB", result.monthly_cost);
    let col3_data_plain = format!("{} RUB", result.individual_cost);
    let col4_data_plain = format!("{} RUB", result.ticket_price);

    let headers = ["Total Trips", "Monthly", "Individual", "Ticket"];
    let data = [
        &col1_data_plain,
        &col2_data_plain,
        &col3_data_plain,
        &col4_data_plain,
    ];

    let col_widths: Vec<usize> = headers
        .iter()
        .zip(data.iter())
        .map(|(h, d)| h.len().max(d.len()))
        .collect();

    let make_border = |left: &str, middle: &str, right: &str| {
        let mut parts = Vec::new();
        for &w in &col_widths {
            parts.push("─".repeat(w + 2));
        }
        format!("{}{}{}", left, parts.join(middle), right)
    };

    let top_border = make_border("╭", "┬", "╮");
    let middle_border = make_border("├", "┼", "┤");
    let bottom_border = make_border("╰", "┴", "╯");

    let header_row = headers
        .iter()
        .enumerate()
        .map(|(i, &h)| format!(" {:^width$} ", h.cyan().bold(), width = col_widths[i]))
        .collect::<Vec<_>>()
        .join("│");

    let data_row = [
        col1_data_plain.yellow(),
        col2_data_plain.yellow(),
        col3_data_plain.yellow(),
        col4_data_plain.yellow(),
    ]
    .iter()
    .enumerate()
    .map(|(i, cell)| format!(" {:^width$} ", cell, width = col_widths[i]))
    .collect::<Vec<_>>()
    .join("│");

    println!("\\n{}", top_border);
    println!("│{}│", header_row);
    println!("{}", middle_border);
    println!("│{}│", data_row);
    println!("{}", bottom_border);

    let message = match result.recommendation {
        rustroika_core::Recommendation::Individual => format!(
            "🚌 Paying per trip is cheaper by {} RUB!",
            result.savings
        )
        .green()
        .bold(),
        rustroika_core::Recommendation::MonthlyPass => format!(
            "💰 Monthly pass saves you {} RUB!",
            result.savings
        )
        .bright_green()
        .bold(),
        rustroika_core::Recommendation::Equal => "⚖️ Both options cost the same".blue().bold(),
    };

    println!("\\n{}\\n", message);

    #[cfg(feature = "detailed-output")]
    if _args.detailed {
        display_detailed_breakdown(&result.breakdown);
    }

    println!("{}", "Note:".bold().underline());
    println!("• Subsequent trips within 90 minutes get 50% discount");
    println!("• Discounted prices are rounded down (e.g., 63 → 31)");
    println!("• Calculation assumes optimal discount usage");
}

fn display_plain_results(result: &CalculationResult, _args: &Args) {
    println!("=== Rustroika Calculation Results ===");
    println!("Total trips per month: {}", result.total_trips);
    println!("Monthly pass cost: {} RUB", result.monthly_cost);
    println!("Individual trips cost: {} RUB", result.individual_cost);
    println!("Single ticket price: {} RUB", result.ticket_price);
    println!();
    println!("Recommendation: {}", result.format_recommendation());
    if result.savings > 0 {
        println!("Savings: {} RUB per month", result.savings);
    }

    #[cfg(feature = "detailed-output")]
    if _args.detailed {
        display_detailed_breakdown_plain(&result.breakdown);
    }

    println!();
    println!("Note:");
    println!("• Subsequent trips within 90 minutes get 50% discount");
    println!("• Discounted prices are rounded down (e.g., 63 → 31)");
    println!("• Calculation assumes optimal discount usage");
}

#[cfg(feature = "detailed-output")]
fn display_detailed_breakdown(breakdown: &rustroika_core::CostBreakdown) {
    println!("{}", "Detailed Breakdown:".bold().underline());
    println!("• Full price trips: {} (cost: {} RUB)", 
             breakdown.full_price_trips, breakdown.full_price_cost);
    println!("• Discounted trips: {} (cost: {} RUB)", 
             breakdown.discounted_trips, breakdown.discounted_cost);
    println!("• Total discount savings: {} RUB", breakdown.discount_savings);
    println!();
}

#[cfg(feature = "detailed-output")]
fn display_detailed_breakdown_plain(breakdown: &rustroika_core::CostBreakdown) {
    println!("Detailed Breakdown:");
    println!("• Full price trips: {} (cost: {} RUB)", 
             breakdown.full_price_trips, breakdown.full_price_cost);
    println!("• Discounted trips: {} (cost: {} RUB)", 
             breakdown.discounted_trips, breakdown.discounted_cost);
    println!("• Total discount savings: {} RUB", breakdown.discount_savings);
}
