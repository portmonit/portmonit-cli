use super::Report;

use std::fs;

use quick_xml::de::from_str;

#[derive(Debug)]
pub enum ParseError {
    InvalidReport {reason: String},
}

pub struct IbkrReportParser {
    file_path: String,
}

impl IbkrReportParser {
    pub fn new(file_path: String) -> IbkrReportParser {
        IbkrReportParser {
            file_path,
        }
    }

    pub fn parse(&self) -> Result<Report::FlexQueryResponse, ParseError> {
        let contents = fs::read_to_string(self.file_path.clone());
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
