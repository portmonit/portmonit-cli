mod broker;
mod tax;

use clap::Parser;

use tax::ua::TaxReportGenerator::*;
use tax::ua::adapters::IbkrAdapter::*;
use tax::ua;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(long, value_name = "input-file")]
    input_file: String,

    #[arg(long, value_name = "input-format")]
    input_format: String,

    #[arg(short, long, value_name = "output-format")]
    output_format: String,
}

fn main() {
    let args = Arguments::parse();

    let ibkr_adapter = IbkrAdapter::new(args.input_file);
    let tax_report_generator = UaTaxReportGenerator::new(
        ua::TaxPolicy::default_tax_policy(),
        Box::new(ibkr_adapter));
    
    let tax_report = tax_report_generator.get_unformal_tax_report();
    match tax_report {
        Ok(tax_report) => {
            println!("Tax report: {:#?}", tax_report);
        },
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

}
