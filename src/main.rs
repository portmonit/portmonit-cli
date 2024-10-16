mod broker;

use broker::ibkr::ReportParser;

use clap::Parser;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, value_name = "file-path")]
    input_file: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    args.input_file.map(|file_path| {
        let parser : Box<dyn ReportParser::IbkrReportParser> = Box::new(broker::ibkr::ReportParser::IbkrReportParserImpl{});
        match parser.parse_report(file_path) {
            Ok(report) => {
                println!("{:#?}", report);
            },
            Err(e) => {
                println!("Error parsing report: {:?}", e);
            },
        }
    });
}
