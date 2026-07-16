use quick_xml::de::from_str;
use std::fs;

use super::report;
use crate::broker::common::report_parser::{
    BrokerRawReport, ReportFormat, ReportParser, ReportParserError,
};

#[derive(Default)]
pub struct IbkrReportParser {}

impl IbkrReportParser {
    pub fn new() -> IbkrReportParser {
        IbkrReportParser {}
    }
}

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
        let _: report::FlexQueryResponse = match from_str(content) {
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
