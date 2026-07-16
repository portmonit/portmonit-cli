use std::path::Path;

use super::report_parser::{ReportFormat, ReportParser, ReportParserError};

#[derive(Debug, Clone, PartialEq)]
pub enum BrokerType {
    IBKR,
    // Add more broker types here as they are supported
    #[allow(dead_code)] // reserved for detect_broker_type's future "unrecognized" case
    Unknown,
}

pub struct BrokerReportParserFactory {}

impl BrokerReportParserFactory {
    pub fn create_parser(broker_type: BrokerType) -> Box<dyn ReportParser> {
        match broker_type {
            BrokerType::IBKR => {
                Box::new(crate::broker::ibkr::report_parser::IbkrReportParser::new())
            }
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
