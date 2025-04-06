use clap::{Parser, arg, command};
use colored::*;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of trips per week
    #[arg(short = 't', long = "trips-week", value_name = "TRIPS")]
    trips_per_week: u32,

    /// Monthly pass cost in RUB
    #[arg(short = 'm', long = "monthly-cost", value_name = "COST")]
    monthly_cost: u32,

    /// Price of a single ticket in RUB
    #[arg(short = 'p', long = "ticket-price", value_name = "PRICE")]
    ticket_price: u32,
}

fn main() {
    let args = Args::parse();

    let total_trips = args.trips_per_week * 4;
    let full_price_count = total_trips.div_ceil(2);
    let discounted_count = total_trips / 2;

    let individual_cost =
        full_price_count * args.ticket_price + discounted_count * (args.ticket_price / 2);

    println!(
        "\n╭{}╴{}╴{}╴{}╴╮",
        "─".repeat(14),
        "─".repeat(10),
        "─".repeat(14),
        "─".repeat(17)
    );
    println!(
        "│ {:14} │ {:10} │ {:14} │ {:10} │",
        "Total trips".cyan().bold(),
        "Monthly".cyan().bold(),
        "Individual".cyan().bold(),
        "Ticket".cyan().bold()
    );
    println!(
        "│ {:14} │ {:>10} │ {:>14} │ {:>10} │",
        format!("{}", total_trips).yellow(),
        format!("{} RUB", args.monthly_cost).yellow(),
        format!("{} RUB", individual_cost).yellow(),
        format!("{} RUB", args.ticket_price).yellow()
    );
    println!(
        "╰{}╴{}╴{}╴{}╯",
        "─".repeat(14),
        "─".repeat(10),
        "─".repeat(14),
        "─".repeat(18)
    );

    let message = match individual_cost.cmp(&args.monthly_cost) {
        std::cmp::Ordering::Less => format!(
            "🚌 Paying per trip is cheaper by {} RUB!",
            args.monthly_cost - individual_cost
        )
        .green()
        .bold(),
        std::cmp::Ordering::Greater => format!(
            "💰 Monthly pass saves you {} RUB!",
            individual_cost - args.monthly_cost
        )
        .bright_green()
        .bold(),
        std::cmp::Ordering::Equal => "⚖️  Both options cost the same".blue().bold(),
    };

    println!("\n{}\n", message);

    // Add explanation of discount logic
    println!("{}", "Note:".bold().underline());
    println!("• Subsequent trips within 90 minutes get 50% discount");
    println!("• Discounted prices are rounded down (e.g., 63 → 31)");
    println!("• Calculation assumes optimal discount usage");
}
