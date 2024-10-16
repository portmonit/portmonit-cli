use super::Report;

use std::fs;

use quick_xml::de::from_str;

#[derive(Debug)]
pub enum ParseError {
    InvalidReport {reason: String},
}

pub trait IbkrReportParser {
    fn parse_report(&self, file_path: String) -> Result<Report::FlexQueryResponse, ParseError>;
}

pub struct IbkrReportParserImpl {}

impl IbkrReportParser for IbkrReportParserImpl {
    fn parse_report(&self, file_path: String) -> Result<Report::FlexQueryResponse, ParseError> {
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(contents) => {
                let ret = from_str(contents.as_str());
                match ret {
                    Ok(report) => {
                        return Ok(report);
                    },
                    Err(e) => {
                        return Err(ParseError::InvalidReport {reason: e.to_string()})
                    }
                }        
            },
            Err(e) => {
                return Err(ParseError::InvalidReport {reason: e.to_string()})
            }
        }
    }
}
