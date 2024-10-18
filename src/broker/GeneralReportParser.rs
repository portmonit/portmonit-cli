
pub trait GeneralReportParser<R, E> {
    fn parse_from_file(&self, file_path: String) -> Result<R, E>;
    fn parse_from_contents(&self, contents: String) -> Result<R, E>;
}
