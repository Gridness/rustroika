use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut trips_per_week: Option<u32> = None;
    let mut monthly_cost: Option<u32> = None;
    let mut ticket_price: Option<u32> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--trips-week" | "-t" => {
                if i + 1 < args.len() {
                    trips_per_week = Some(args[i + 1].parse().expect("Invalid trips per week"));
                    i += 2;
                } else {
                    eprintln!("{}", "Missing value for trips per week".red());
                    std::process::exit(1);
                }
            }
            "--monthly-cost" | "-m" => {
                if i + 1 < args.len() {
                    monthly_cost = Some(args[i + 1].parse().expect("Invalid monthly cost"));
                    i += 2;
                } else {
                    eprintln!("{}", "Missing value for monthly cost".red());
                    std::process::exit(1);
                }
            }
            "--ticket-price" | "-p" => {
                if i + 1 < args.len() {
                    ticket_price = Some(args[i + 1].parse().expect("Invalid ticket price"));
                    i += 2;
                } else {
                    eprintln!("{}", "Missing value for ticket price".red());
                    std::process::exit(1);
                }
            }
            _ => {
                eprintln!("{}", format!("Unknown argument: {}", args[i]).red());
                i += 1;
            }
        }
    }

    if trips_per_week.is_none() || monthly_cost.is_none() || ticket_price.is_none() {
        eprintln!(
            "{}",
            format!(
                "Usage: {} --trips-week|-t <trips/week> --monthly-cost|-m <monthly_cost> --ticket-price|-p <ticket_price>",
                args[0]
            )
            .red()
        );
        std::process::exit(1);
    }

    let trips_per_week = trips_per_week.unwrap();
    let monthly_cost = monthly_cost.unwrap();
    let ticket_price = ticket_price.unwrap();

    let total_trips = trips_per_week * 4;
    let full_price_count = (total_trips + 1) / 2;
    let discounted_count = total_trips / 2;

    let individual_cost = full_price_count * ticket_price + discounted_count * (ticket_price / 2);

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
        format!("{} RUB", monthly_cost).yellow(),
        format!("{} RUB", individual_cost).yellow(),
        format!("{} RUB", ticket_price).yellow()
    );
    println!(
        "╰{}╴{}╴{}╴{}╯",
        "─".repeat(14),
        "─".repeat(10),
        "─".repeat(14),
        "─".repeat(18)
    );

    let message = match individual_cost.cmp(&monthly_cost) {
        std::cmp::Ordering::Less => format!(
            "🚌 Paying per trip is cheaper by {} RUB!",
            monthly_cost - individual_cost
        )
        .green()
        .bold(),
        std::cmp::Ordering::Greater => format!(
            "💰 Monthly pass saves you {} RUB!",
            individual_cost - monthly_cost
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
