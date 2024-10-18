use crate::broker::GeneralReportParser::GeneralReportParser;

use super::Report;

use std::fs;

use quick_xml::de::from_str;

#[derive(Debug)]
pub enum ParseError {
    InvalidReport {reason: String},
}

pub type IbkrReportParserGeneral = dyn GeneralReportParser<Report::FlexQueryResponse, ParseError>;

pub struct IbkrReportParser {}

impl GeneralReportParser<Report::FlexQueryResponse, ParseError> for IbkrReportParser {
    fn parse_from_file(&self, file_path: String) -> Result<Report::FlexQueryResponse, ParseError> {
        let contents = fs::read_to_string(file_path);
        match contents {
            Ok(contents) => {
                return self.parse_from_contents(contents); 
            },
            Err(e) => {
                return Err(ParseError::InvalidReport {reason: e.to_string()})
            }
        }
    }
    fn parse_from_contents(&self, contents: String) -> Result<Report::FlexQueryResponse, ParseError> {
        let ret = from_str(contents.as_str());
        match ret {
            Ok(report) => {
                return Ok(report);
            },
            Err(e) => {
                return Err(ParseError::InvalidReport {reason: e.to_string()})
            }
        }
    }
}
