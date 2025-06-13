# Examine

[![Crates.io](https://img.shields.io/crates/v/examine)](https://crates.io/crates/examine)
[![Documentation](https://docs.rs/examine/badge.svg)](https://docs.rs/examine)
[![Crates.io Downloads](https://img.shields.io/crates/d/examine)](https://crates.io/crates/examine)
[![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/examine)](https://crates.io/crates/examine)
[![Open Source](https://img.shields.io/badge/open-source-brightgreen)](https://github.com/anistark/examine)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
![maintenance-status](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

Simple project detection and analysis. Pass a path, get comprehensive project info.

> Highly experimental at the moment!

## Installation

```toml
[dependencies]
examine = "0.1.0"
```

## Usage

```rust
use examine::examine;

let info = examine(".").unwrap();
println!("Language: {}", info.language);
println!("Version: {}", info.language_version.unwrap_or("Unknown".to_string()));
println!("Status: {}", info.language_status);
println!("Framework: {}", info.framework.unwrap_or("None".to_string()));
```

## Current Status

| Language | Project Detection | Version Detection | EOL Tracking | Frameworks Supported |
| --- | --- | --- | --- | --- |
| ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) | ✅ | ✅ | ✅ | Axum, Actix Web, Rocket, Warp, Bevy, Clap |
| ![JavaScript](https://img.shields.io/badge/javascript-%23323330.svg?style=for-the-badge&logo=javascript&logoColor=%23F7DF1E) | ✅ | ✅ | ✅ | React, Vue, Angular, Svelte, Express, Next.js, Nuxt |
|![Go](https://img.shields.io/badge/go-%2300ADD8.svg?style=for-the-badge&logo=go&logoColor=white) | ✅ | ✅ | ✅ | Gin, Echo, Fiber, Gorilla Mux |
| ![Python](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54) | ✅ | ✅ | ✅ | Django, Flask, FastAPI |
| ![Java](https://img.shields.io/badge/java-%23ED8B00.svg?style=for-the-badge&logo=openjdk&logoColor=white) | ✅ | WIP | WIP | |
| ![PHP](https://img.shields.io/badge/php-%23777BB4.svg?style=for-the-badge&logo=php&logoColor=white)| ✅ | TODO | | |
| ![Ruby](https://img.shields.io/badge/ruby-%23CC342D.svg?style=for-the-badge&logo=ruby&logoColor=white)| ✅ | TODO | | |
| ![Swift](https://img.shields.io/badge/swift-F54A2A?style=for-the-badge&logo=swift&logoColor=white)| ✅ | TODO | | |
| ![Dart](https://img.shields.io/badge/dart-%230175C2.svg?style=for-the-badge&logo=dart&logoColor=white)| ✅ | TODO | | |
| ![C](https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white)| ✅ | TODO | | |
| ![C++](https://img.shields.io/badge/c++-%2300599C.svg?style=for-the-badge&logo=c%2B%2B&logoColor=white)| ✅ | TODO | | |
| ![C#](https://img.shields.io/badge/c%23-%23239120.svg?style=for-the-badge&logo=csharp&logoColor=white)| ✅ | TODO | | |
| ![Elixir](https://img.shields.io/badge/elixir-%234B275F.svg?style=for-the-badge&logo=elixir&logoColor=white)| ✅ | TODO | | |
| ![Haskell](https://img.shields.io/badge/Haskell-5e5086?style=for-the-badge&logo=haskell&logoColor=white)| ✅ | TODO | | |
| ![Clojure](https://img.shields.io/badge/Clojure-%23Clojure.svg?style=for-the-badge&logo=Clojure&logoColor=Clojure)| ✅ | TODO | | |

## Example Output

```sh
🔍 Project Analysis
==================
📁 Path: ./my-web-service
📦 Name: my-web-service
💻 Language: Rust
📋 Version: 1.75.0
⚡ Status: ✅ Supported
🚀 Framework: Axum
   Version: 0.7.0
   Type: Web Framework
   Popular: Yes
   Alternatives: Actix Web, Warp, Rocket

✨ Summary: Rust + v1.75.0 + Axum v0.7.0
```

## Language Status Guide

- **✅ Supported** - Actively maintained, safe to use
- **⚠️ Ending Soon** - Will reach EOL within 6 months  
- **❌ End of Life** - No longer supported, consider upgrading
- **❓ Unknown** - Status not tracked or version not detected

## CLI Tool

> The CLI is only for testing the library and not intended as a distribution. Feel free to write your own CLI tool if you want one.

```sh
cargo run --example cli_tool analyze .
cargo run --example cli_tool analyze /path/to/project
```

## API Reference

### `ProjectInfo` struct

```rust
pub struct ProjectInfo {
    pub language: String,
    pub language_version: Option<String>,
    pub language_status: LanguageStatus,
    pub framework: Option<String>,
    pub framework_version: Option<String>,
    pub framework_details: Option<FrameworkDetails>,
    pub project_name: Option<String>,
    pub project_path: String,
}
```

### `LanguageStatus` enum

```rust
pub enum LanguageStatus {
    Supported,
    EndingSoon { date: String },
    EndOfLife { date: String },
    Unknown,
}
```

## How it works

1. **Language Detection** - Looks for project files (`Cargo.toml`, `package.json`, etc.)
2. **Version Detection** - Parses project files and version config files
3. **Framework Detection** - Analyzes dependencies in project files  
4. **Status Lookup** - Checks against known End of Life databases

## Contributing

Contributions welcome! Areas where help is needed:

- More languages
- More frameworks. Single Data Source?
- Better version detection
- Updated EOL information. Automated update to state.
- New Ideas

## License

MIT License - see [LICENSE](./LICENSE) file for details.
