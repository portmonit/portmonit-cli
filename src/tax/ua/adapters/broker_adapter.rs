use crate::broker::common::broker_report_parser_factory::{BrokerReportParserFactory, BrokerType};
use crate::broker::common::report_parser::{BrokerRawReport, ReportParser};

// Abstract adapter that can work with any broker implementation
pub struct BrokerAdapter {
    broker_type: BrokerType,
    file_path: String,
    parser: Box<dyn ReportParser>,
}

impl BrokerAdapter {
    // Create adapter by explicitly specifying broker type
    pub fn new(broker_type: BrokerType, file_path: String) -> Self {
        let parser = BrokerReportParserFactory::create_parser(broker_type.clone());
        BrokerAdapter {
            broker_type,
            file_path,
            parser,
        }
    }

    // Auto-detect broker type from file
    pub fn from_file(file_path: String) -> Result<Self, String> {
        let broker_type = match BrokerReportParserFactory::detect_broker_type(&file_path) {
            Ok(broker_type) => broker_type,
            Err(e) => return Err(format!("Failed to detect broker type: {}", e)),
        };

        Ok(Self::new(broker_type, file_path))
    }

    pub fn get_broker_type(&self) -> &BrokerType {
        &self.broker_type
    }

    // Parse report using the appropriate parser
    pub fn parse_report(&self) -> Result<BrokerRawReport, String> {
        match self.parser.parse_from_file(&self.file_path) {
            Ok(report) => Ok(report),
            Err(e) => Err(format!("Failed to parse broker report: {}", e)),
        }
    }
}
