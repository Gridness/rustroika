pub mod args;
pub mod console;

use std::fs;

use colored::*;

use args::{Args, ConfigAction, MainCommand};
use console::run_console;

use crate::{
    config::{get_config_path, load_config, remove_config_value, save_config, update_config},
    error,
    export::{ExportData, export_data},
    prelude::*,
};

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Some(MainCommand::Config { action }) => match action {
            ConfigAction::Set { key, value } => {
                update_config(&key, &value)?;
                println!("{} Configuration updated", "✓".green());
            }
            ConfigAction::Remove { key } => {
                let mut config = load_config();
                let parts: Vec<&str> = key.split('.').collect();
                if remove_config_value(&mut config, &parts) {
                    save_config(&config)?;
                    println!("{} Configuration value removed", "✓".green());
                } else {
                    println!("{} No value found at {}", "⚠".red(), key);
                }
            }
            ConfigAction::Purge => {
                if let Some(path) = get_config_path() {
                    fs::remove_file(path)?;
                    println!("{} Configuration file purged", "✓".green());
                }
            }
        },
        Some(MainCommand::Run {
            ref params,
            ref export,
        }) => {
            let config = load_config();

            let trips_per_week = params
                .trips_per_week
                .or_else(|| {
                    config
                        .get("defaults.trips-per-week")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing trips-per-week"))?;

            let monthly_cost = params
                .monthly_cost
                .or_else(|| {
                    config
                        .get("defaults.monthly-cost")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing monthly-cost"))?;

            let ticket_price = params
                .ticket_price
                .or_else(|| {
                    config
                        .get("defaults.ticket-price")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing ticket-price"))?;

            let total_trips = trips_per_week * 4;
            let full_price_count = total_trips.div_ceil(2);
            let discounted_count = total_trips / 2;

            let individual_cost =
                full_price_count * ticket_price + discounted_count * (ticket_price / 2);

            let col1_data_plain = total_trips.to_string();
            let col2_data_plain = format!("{} RUB", monthly_cost);
            let col3_data_plain = format!("{} RUB", individual_cost);
            let col4_data_plain = format!("{} RUB", ticket_price);

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

            if let Some(format) = export {
                let e_data =
                    ExportData::new(trips_per_week, monthly_cost, ticket_price, individual_cost);
                if let Err(e) = export_data(&e_data, &format) {
                    eprintln!("{} Failed to export: {}", "✗".red(), e);
                }
            }
        }
        Some(_) => {
            let err: Result<()> = Err(error::Error::InteractiveModeRequired(args.command.unwrap()));
            eprintln!("{} {}", "✗".red(), err.unwrap_err());
        }
        None => {
            run_console()?;
            return Ok(());
        }
    }

    Ok(())
}
