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

    let col1_data_plain = total_trips.to_string();
    let col2_data_plain = format!("{} RUB", args.monthly_cost);
    let col3_data_plain = format!("{} RUB", individual_cost);
    let col4_data_plain = format!("{} RUB", args.ticket_price);

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

    println!("\n{}", top_border);
    println!("│{}│", header_row);
    println!("{}", middle_border);
    println!("│{}│", data_row);
    println!("{}", bottom_border);

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
