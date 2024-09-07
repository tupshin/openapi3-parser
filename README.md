
# OpenAPI 3.0 Parser Library

[![Crates.io](https://img.shields.io/crates/v/openapi3-parser.svg)](https://crates.io/crates/openapi3-parser)
[![Documentation](https://docs.rs/openapi3-parser/badge.svg)](https://docs.rs/openapi3-parser)

OpenAPI 3.0 Parser Library is a Rust crate designed to parse OpenAPI (Swagger) 3.0 specifications in both JSON and YAML 
formats. This library allows developers to easily handle OpenAPI spec files and extract the information programmatically.

## Features

- Supports parsing OpenAPI 3.0 specifications in both JSON and YAML.
- Extract and manipulate OpenAPI components such as paths, operations, parameters, responses, etc.
- Written in Rust for high performance and safety.
  
## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
openapi3-parser = "0.1.0"
```

## Usage

```rust
use openapi3_parser::OpenApiSpec;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

fn main() -> Result<()> {
    // Load OpenAPI spec file
    let mut file = File::open("openapi_spec.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the OpenAPI spec
    let openapi_spec: OpenApiSpec = serde_yaml::from_str(&contents)?;
    
    // Print the title of the API
    if let Some(info) = openapi_spec.info {
        if let Some(title) = info.title {
            println!("API Title: {}", title);
        }
    }

    Ok(())
}
```

## Examples

You can parse a file in either YAML or JSON format:

```rust
use openapi3_parser::OpenApiSpec;
use serde_yaml;
use serde_json;
use std::fs::File;
use std::io::Read;
use anyhow::Result;

fn load_openapi_spec(path: &str) -> Result<OpenApiSpec> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if path.ends_with(".yaml") || path.ends_with(".yml") {
        Ok(serde_yaml::from_str(&contents)?)
    } else if path.ends_with(".json") {
        Ok(serde_json::from_str(&contents)?)
    } else {
        Err(anyhow::anyhow!("Unsupported file format"))
    }
}

fn main() -> Result<()> {
    let spec = load_openapi_spec("path/to/openapi_spec.yaml")?;
    println!("{:?}", spec);
    Ok(())
}
```

### Documentation

Full documentation is available on [docs.rs](https://docs.rs/openapi3-parser).

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or need help, please [open an issue on GitHub](https://github.com/ayonsaha2011/openapi3-parser/issues).

For additional support, you can contact me at [ayonsaha2011@gmail.com](mailto:ayonsaha2011@gmail.com).

## Donation

If you find this library useful and would like to support its ongoing development, consider making a donation. Your support is greatly appreciated!

[Buy me a coffee](https://www.buymeacoffee.com/ayonsaha2011)
