use std::path::PathBuf;

use chrono::Local;
use clap_derive::ValueEnum;
use colored::*;
use rust_xlsxwriter::Format;
use rust_xlsxwriter::workbook::Workbook;
use serde::Serialize;

use crate::prelude::*;

#[derive(Clone, ValueEnum, Debug)]
pub enum ExportFormat {
    Csv,
    Xlsx,
    Json,
}

#[derive(Debug, Serialize)]
pub struct ExportData {
    pub total_trips: u32,
    pub monthly_cost: u32,
    pub individual_cost: u32,
    pub ticket_price: u32,
    pub recommendation: String,
    pub savings: i64,
}

impl ExportData {
    pub fn new(
        trips_per_week: u32,
        monthly_cost: u32,
        ticket_price: u32,
        individual_cost: u32,
    ) -> Self {
        let savings = monthly_cost as i64 - individual_cost as i64;
        let recommendation = match individual_cost.cmp(&monthly_cost) {
            std::cmp::Ordering::Less => {
                format!("Paying per trip is cheaper by {} RUB", savings.abs())
            }
            std::cmp::Ordering::Greater => format!("Monthly pass saves you {} RUB", savings.abs()),
            std::cmp::Ordering::Equal => "Both options cost the same".to_string(),
        };

        ExportData {
            total_trips: trips_per_week * 4,
            monthly_cost,
            individual_cost,
            ticket_price,
            recommendation,
            savings,
        }
    }
}

pub fn generate_filename(format: &ExportFormat) -> String {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    match format {
        ExportFormat::Csv => format!("rustroika_report_{}.csv", timestamp),
        ExportFormat::Xlsx => format!("rustroika_report_{}.xlsx", timestamp),
        ExportFormat::Json => format!("rustroika_report_{}.json", timestamp),
    }
}

pub fn export_data(data: &ExportData, format: &ExportFormat) -> Result<()> {
    let filename = generate_filename(format);
    let path = PathBuf::from(&filename);

    match format {
        ExportFormat::Csv => {
            let mut writer = csv::Writer::from_path(&path)?;
            writer.serialize(data)?;
            writer.flush()?;
        }
        ExportFormat::Xlsx => {
            let mut workbook = Workbook::new();
            let worksheet = workbook.add_worksheet();
            let bold = Format::new().set_bold();
            let money_format = Format::new().set_num_format("[$₽-ru-RU] #,##0.00");
            let center_aligned = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

            worksheet.write_with_format(0, 0, "Total Trips", &bold)?;
            worksheet.write_with_format(0, 1, "Monthly Cost", &bold)?;
            worksheet.write_with_format(0, 2, "Individual Cost", &bold)?;
            worksheet.write_with_format(0, 3, "Ticket Price", &bold)?;
            worksheet.write_with_format(0, 4, "Recommendation", &bold)?;
            worksheet.write_with_format(0, 5, "Savings", &bold)?;

            worksheet.write_number(1, 0, data.total_trips as f64)?;
            worksheet.write_with_format(1, 1, data.monthly_cost as f64, &money_format)?;
            worksheet.write_with_format(1, 2, data.individual_cost as f64, &money_format)?;
            worksheet.write_with_format(1, 3, data.ticket_price as f64, &money_format)?;
            worksheet.write_string(1, 4, &data.recommendation)?;
            worksheet.write_with_format(1, 5, data.savings as f64, &money_format)?;

            worksheet.autofit();
            worksheet.set_column_format(4, &center_aligned)?;

            workbook.save(&path)?;
        }
        ExportFormat::Json => {
            let json = serde_json::to_string_pretty(data)?;
            std::fs::write(&path, json)?;
        }
    }

    println!("{} Exported to: {}", "✓".green(), filename.cyan());
    Ok(())
}
