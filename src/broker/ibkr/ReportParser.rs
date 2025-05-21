use quick_xml::de::from_str;
use std::fs;

use super::Report;
use crate::broker::common::ReportParser::{
    BrokerRawReport, ReportFormat, ReportParser, ReportParserError,
};

// Keep the old error type for backward compatibility
#[derive(Debug)]
pub enum ParseError {
    InvalidReport { reason: String },
}

pub struct IbkrReportParser {
    file_path: String,
}

impl IbkrReportParser {
    pub fn new(file_path: String) -> IbkrReportParser {
        IbkrReportParser { file_path }
    }

    // Legacy method for compatibility
    pub fn parse(&self) -> Result<Report::FlexQueryResponse, ParseError> {
        let contents = match fs::read_to_string(self.file_path.clone()) {
            Ok(contents) => contents,
            Err(e) => {
                return Err(ParseError::InvalidReport {
                    reason: e.to_string(),
                })
            }
        };

        match from_str(contents.as_str()) {
            Ok(report) => Ok(report),
            Err(e) => Err(ParseError::InvalidReport {
                reason: e.to_string(),
            }),
        }
    }
}

// Implement the new generic parser interface
impl ReportParser for IbkrReportParser {
    fn parse_from_file(&self, file_path: &str) -> Result<BrokerRawReport, ReportParserError> {
        let contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(e) => {
                return Err(ReportParserError::FileReadError {
                    reason: e.to_string(),
                })
            }
        };

        self.parse_from_content(&contents)
    }

    fn parse_from_content(&self, content: &str) -> Result<BrokerRawReport, ReportParserError> {
        // Validate that content is valid XML and can be parsed as IBKR report
        let _: Report::FlexQueryResponse = match from_str(content) {
            Ok(report) => report,
            Err(e) => {
                return Err(ReportParserError::ParseError {
                    reason: e.to_string(),
                })
            }
        };

        // If validation passed, return the raw content
        Ok(BrokerRawReport {
            content: content.to_string(),
            format: ReportFormat::XML,
        })
    }

    fn supported_formats(&self) -> Vec<ReportFormat> {
        vec![ReportFormat::XML]
    }
}
