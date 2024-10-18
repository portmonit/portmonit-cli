mod broker;
mod tax;

use clap::Parser;

use broker::ibkr;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, value_name = "file-path")]
    input_file: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    args.input_file.map(|file_path: String| {
        let parser : Box<ibkr::ReportParser::IbkrReportParserGeneral> = Box::new(ibkr::ReportParser::IbkrReportParser{});
        match parser.parse_from_file(file_path) {
            Ok(report) => {
                println!("{:#?}", report);
            },
            Err(e) => {
                println!("Error parsing report: {:?}", e);
            },
        }
    });
}
