// Regression guard for #1: `cargo build` must stay free of the warning
// categories it used to flood the terminal with (unused imports, deprecated
// API usage, dead code, non-snake-case module names).
#![deny(unused_imports, deprecated, dead_code, nonstandard_style)]

mod broker;
mod tax;

use clap::Parser;

use tax::ua;
use tax::ua::adapters::ibkr_adapter::*;
use tax::ua::tax_report_generator::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(long, value_name = "input-file")]
    input_file: String,

    #[arg(long, value_name = "input-format", default_value = "auto")]
    input_format: String,

    #[arg(short, long, value_name = "output-format")]
    output_format: String,
}

fn main() {
    let args = Arguments::parse();

    // Determine broker type automatically or from input format
    let adapter = if args.input_format.to_lowercase() == "auto" {
        // Try to auto-detect broker type
        match IbkrAdapter::from_file(args.input_file.clone()) {
            Ok(adapter) => adapter,
            Err(e) => {
                eprintln!("Failed to auto-detect broker type: {}", e);
                eprintln!("Falling back to IBKR format");
                IbkrAdapter::new(args.input_file.clone())
            }
        }
    } else if args.input_format.to_lowercase() == "ibkr" {
        // Explicitly use IBKR format
        IbkrAdapter::new(args.input_file.clone())
    } else {
        eprintln!("Unsupported input format: {}", args.input_format);
        eprintln!("Falling back to IBKR format");
        IbkrAdapter::new(args.input_file)
    };

    // Create tax report generator with the adapter
    let tax_report_generator =
        UaTaxReportGenerator::new(ua::tax_policy::default_tax_policy(), Box::new(adapter));

    // Generate tax report
    let tax_report = tax_report_generator.get_unformal_tax_report();
    match tax_report {
        Ok(tax_report) => {
            println!("Tax report: {:#?}", tax_report);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
