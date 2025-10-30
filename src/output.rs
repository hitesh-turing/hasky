use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

/// Output format options for hash encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Hex,
    Base64,
    Raw,
}

impl OutputFormat {
    /// Format hash bytes according to the selected format
    /// For raw format, returns empty string (caller should write bytes directly)
    pub fn format_bytes(&self, bytes: &[u8], uppercase: bool) -> String {
        match self {
            OutputFormat::Hex => {
                let hex_str = hex::encode(bytes);
                if uppercase {
                    hex_str.to_uppercase()
                } else {
                    hex_str
                }
            }
            OutputFormat::Base64 => general_purpose::STANDARD.encode(bytes),
            OutputFormat::Raw => {
                // For raw bytes, we'll write directly in the caller
                // Return empty string as a placeholder
                String::new()
            }
        }
    }

    /// Check if this format requires direct byte output
    pub fn is_raw(&self) -> bool {
        matches!(self, OutputFormat::Raw)
    }
}

/// JSON output structure for hash results
#[derive(Debug, Serialize)]
pub struct HashJsonOutput {
    pub algo: String,
    pub source: String,
    pub digest: String,
    pub bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// JSON output structure for batch hash results
#[derive(Debug, Serialize)]
pub struct BatchHashJsonOutput {
    pub algo: String,
    pub results: Vec<HashJsonOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}
