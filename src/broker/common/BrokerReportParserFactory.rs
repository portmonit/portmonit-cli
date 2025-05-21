use std::path::Path;

use super::ReportParser::{ReportFormat, ReportParser, ReportParserError};

#[derive(Debug, Clone, PartialEq)]
pub enum BrokerType {
    IBKR,
    // Add more broker types here as they are supported
    Unknown,
}

pub struct BrokerReportParserFactory {}

impl BrokerReportParserFactory {
    pub fn create_parser(broker_type: BrokerType) -> Box<dyn ReportParser> {
        match broker_type {
            BrokerType::IBKR => Box::new(crate::broker::ibkr::ReportParser::IbkrReportParser::new(
                String::new(),
            )),
            // Add more cases for other broker types
            _ => panic!("Unsupported broker type: {:?}", broker_type),
        }
    }

    pub fn detect_broker_type(file_path: &str) -> Result<BrokerType, ReportParserError> {
        let extension = match Path::new(file_path).extension() {
            Some(ext) => ext.to_str().unwrap_or("").to_lowercase(),
            None => {
                return Err(ReportParserError::ParseError {
                    reason: "File has no extension".to_string(),
                })
            }
        };

        // For now, simple detection based on file extension
        // In real application, this would be more sophisticated
        match extension.as_str() {
            "xml" => {
                // Could do additional content check here
                Ok(BrokerType::IBKR)
            }
            _ => Err(ReportParserError::UnsupportedFormat {
                format: ReportFormat::Custom(extension),
            }),
        }
    }
}
