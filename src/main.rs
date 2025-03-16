fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <trips_per_week> <monthly_cost> <ticket_price>",
            args[0]
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

    println!("Total trips per month: {}", total_trips);
    println!("Individual cost: {}", individual_cost);
    println!("Monthly pass cost: {}", monthly_cost);

    match individual_cost.cmp(&monthly_cost) {
        std::cmp::Ordering::Less => println!("Paying per trip is cheaper!"),
        std::cmp::Ordering::Greater => println!("Buying the monthly pass is cheaper!"),
        std::cmp::Ordering::Equal => println!("Both options cost the same."),
    }
}
