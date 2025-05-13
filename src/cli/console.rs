use std::{fs, process::exit};

use clap::Parser;
use colored::*;
use directories::ProjectDirs;
use rustyline::{Editor, history::FileHistory};

use crate::{
    cli::args::ConsoleCommand,
    config::{get_config_path, load_config, remove_config_value, save_config, update_config},
    error,
    export::{ExportData, export_data},
    prelude::*,
};

use super::args::{ConfigAction, MainCommand};

pub fn run_console() -> Result<()> {
    let mut r1 = Editor::<(), FileHistory>::new().expect("Could not initialize readline");
    r1.load_history(
        ProjectDirs::from("", "", "rustroika")
            .map(|proj_dirs| proj_dirs.config_dir().join("history.txt"))
            .unwrap()
            .as_path(),
    )
    .ok();

    let top_str = format!(
        r#"
    ┌──────────────────────────────────────────────┐
    │            Rustroika Interactive Console     │
    │   Version: {:<10}                        │
    │   Type 'help' for commands                   │
    └──────────────────────────────────────────────┘
    "#,
        env!("CARGO_PKG_VERSION")
    );
    println!("{}", top_str.bright_blue());

    loop {
        let readline = r1.readline("rustroika> ");
        match readline {
            Ok(line) => {
                let _ = r1.add_history_entry(&line);
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                let args = shlex::split(line).unwrap_or_default();
                let args_with_bin =
                    std::iter::once("rustroika").chain(args.iter().map(|s| s.as_str()));
                match ConsoleCommand::try_parse_from(args_with_bin) {
                    Ok(cmd) => handle_console_command(cmd),
                    Err(e) => e.print().expect("Error printing error"),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C: Use 'exit' to quit");
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D: Use 'exit' to quit");
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn handle_console_command(cmd: ConsoleCommand) {
    match cmd.command {
        Some(MainCommand::Run {
            ref params,
            ref export,
        }) => {
            let config = load_config();

            let trips_per_week = params
                .trips_per_week
                .or_else(|| {
                    config
                        .get("defaults.total-trips-per-week")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing trips-per-week"))
                .unwrap();

            let monthly_cost = params
                .monthly_cost
                .or_else(|| {
                    config
                        .get("defaults.monthly-cost")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing monthly-cost"))
                .unwrap();

            let ticket_price = params
                .ticket_price
                .or_else(|| {
                    config
                        .get("defaults.ticket-price")
                        .and_then(|v| v.as_u64().map(|n| n as u32))
                })
                .ok_or(anyhow::anyhow!("Missing ticket-price"))
                .unwrap();

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
        Some(MainCommand::Config { action }) => match action {
            ConfigAction::Set { key, value } => {
                let _ = update_config(&key, &value);
                println!("{} Configuration updated", "✓".green());
            }
            ConfigAction::Remove { key } => {
                let mut config = load_config();
                let parts: Vec<&str> = key.split('.').collect();
                if remove_config_value(&mut config, &parts) {
                    let _ = save_config(&config);
                    println!("{} Configuration value removed", "✓".green());
                } else {
                    println!("{} No value found at {}", "⚠".red(), key);
                }
            }
            ConfigAction::Purge => {
                if let Some(path) = get_config_path() {
                    let _ = fs::remove_file(path);
                    println!("{} Configuration file purged", "✓".green());
                }
            }
        },
        Some(MainCommand::Exit) => {
            println!("exit");
            exit(0);
        }
        Some(MainCommand::Clear) => {
            println!("{esc}c", esc = 27 as char);
        }
        None if cmd.help => print_help(),
        None => eprintln!("{}", error::Error::NoCommandProvided),
    }
}

fn print_help() {
    println!(
        r#"
    {}
    
    {} [options]     - Run the calculation
      Options:
        -t, --trips-week <TRIPS>  Number of trips per week
        -m, --monthly-cost <COST> Monthly pass cost
        -p, --ticket-price <PRICE> Single ticket price
        --export <FORMAT>         Export format (csv, xlsx, json)

    {} <action>   - Manage configuration
      Actions:
        set <key>=<value>    Set config value
        remove <key>         Remove config value
        purge                Remove all config
        
    {}             - Clear the screen
    {}             - Exit the console
    {}             - Show this help
    
    {}
      run -t "(6+2)*3" -m 3070 -p 63
      config set defaults.monthly-cost 3070
      export csv
    "#,
        "Available Commands:".bold(),
        "run".cyan(),
        "config".cyan(),
        "clear".cyan(),
        "exit".cyan(),
        "help".cyan(),
        "Examples:".yellow()
    );
}
