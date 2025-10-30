use anyhow::{anyhow, Result};
use std::fmt;

/// Supported hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Sha1,
    Sha256,
    Sha512,
    Blake3,
    Md5,
}

impl Algorithm {
    /// Parse algorithm from string (case-insensitive)
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "sha1" => Ok(Algorithm::Sha1),
            "sha256" => Ok(Algorithm::Sha256),
            "sha512" => Ok(Algorithm::Sha512),
            "blake3" => Ok(Algorithm::Blake3),
            "md5" => Ok(Algorithm::Md5),
            _ => Err(anyhow!("Unsupported algorithm: {}", s)),
        }
    }

    /// Check if algorithm is considered insecure
    pub fn is_insecure(&self) -> bool {
        matches!(self, Algorithm::Sha1 | Algorithm::Md5)
    }

    /// Get display name for the algorithm
    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Sha1 => "sha1",
            Algorithm::Sha256 => "sha256",
            Algorithm::Sha512 => "sha512",
            Algorithm::Blake3 => "blake3",
            Algorithm::Md5 => "md5",
        }
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
