use pdf_extract;
use crate::pdf::pdf_extract::OutputError;

pub fn get_text(path: &str) -> Result<String, OutputError> {
    return pdf_extract::extract_text(path);
}
