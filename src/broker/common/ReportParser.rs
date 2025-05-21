use std::fmt;

// Generic Report structure that different brokers can convert their data into
#[derive(Debug, Clone)]
pub struct BrokerRawReport {
    pub content: String,
    pub format: ReportFormat,
}

#[derive(Debug, Clone)]
pub enum ReportFormat {
    XML,
    CSV,
    JSON,
    PDF,
    Custom(String),
}

#[derive(Debug)]
pub enum ReportParserError {
    FileReadError { reason: String },
    ParseError { reason: String },
    UnsupportedFormat { format: ReportFormat },
}

impl fmt::Display for ReportParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReportParserError::FileReadError { reason } => {
                write!(f, "Failed to read report file: {}", reason)
            }
            ReportParserError::ParseError { reason } => {
                write!(f, "Failed to parse report: {}", reason)
            }
            ReportParserError::UnsupportedFormat { format } => {
                write!(f, "Unsupported report format: {:?}", format)
            }
        }
    }
}

// Common trait for all broker report parsers
pub trait ReportParser {
    // Parse a report from a file path
    fn parse_from_file(&self, file_path: &str) -> Result<BrokerRawReport, ReportParserError>;

    // Parse a report from a string content
    fn parse_from_content(&self, content: &str) -> Result<BrokerRawReport, ReportParserError>;

    // Get supported formats by this parser
    fn supported_formats(&self) -> Vec<ReportFormat>;
}
