use colored::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "{}",
            format!(
                "Usage: {} <trips/week> <monthly_cost> <ticket_price>",
                args[0]
            )
            .red()
        );
        std::process::exit(1);
    }

    let trips_per_week: u32 = args[1].parse().expect("Invalid trips per week");
    let monthly_cost: u32 = args[2].parse().expect("Invalid monthly cost");
    let ticket_price: u32 = args[3].parse().expect("Invalid ticket price");

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
