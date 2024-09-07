pub mod open_api;
use open_api::OpenApiSpec;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenApiError {
    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Failed to parse YAML: {0}")]
    YamlParseError(#[from] serde_yaml::Error),

    #[error("Unsupported file format. Only .json and .yaml/.yml files are supported.")]
    UnsupportedFormat,
}

// Enum to handle supported file types
enum FileType {
    Json,
    Yaml,
}

/// Determines the file type based on the file extension (.json or .yaml)
fn determine_file_type(file_path: &Path) -> Result<FileType, OpenApiError> {
    match file_path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => Ok(FileType::Json),
        Some("yaml") | Some("yml") => Ok(FileType::Yaml),
        _ => Err(OpenApiError::UnsupportedFormat),
    }
}

/// Parses a JSON or YAML OpenAPI specification file into the OpenAPI struct
pub fn parse_openapi_file(file_path: &str) -> Result<OpenApiSpec, OpenApiError> {
    // Determine the file type (JSON or YAML)
    let file_type = determine_file_type(Path::new(file_path))?;

    // Read the file contents
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the file based on its type
    match file_type {
        FileType::Json => {
            let openapi: OpenApiSpec = serde_json::from_str(&contents)?;
            Ok(openapi)
        }
        FileType::Yaml => {
            let openapi: OpenApiSpec = serde_yaml::from_str(&contents)?;
            Ok(openapi)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let result = parse_openapi_file("test-data/openapi.json");
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_yaml() {
        let result = parse_openapi_file("test-data/openapi.yaml");
        assert!(result.is_ok());
    }
}
