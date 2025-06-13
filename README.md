# Examine

[![Crates.io](https://img.shields.io/crates/v/examine)](https://crates.io/crates/examine)
[![Documentation](https://docs.rs/examine/badge.svg)](https://docs.rs/examine)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Simple project detection and analysis. Pass a path, get comprehensive project info.

> Highly experimental at the moment!

## Usage

```rust
use examine::examine;

let info = examine(".").unwrap();
println!("Language: {}", info.language);
println!("Version: {}", info.language_version.unwrap_or("Unknown".to_string()));
println!("Status: {}", info.language_status);
println!("Framework: {}", info.framework.unwrap_or("None".to_string()));
```

## Installation

```toml
[dependencies]
examine = "0.1.0"
```

## What it detects

**Languages**: Rust, JavaScript, Go, Python, Java, PHP, Ruby, Swift, Dart, C/C++, C#, Elixir, Haskell, Clojure

**Frameworks**:
- **Rust**: Axum, Actix Web, Rocket, Warp, Bevy, Clap
- **JavaScript**: React, Vue, Angular, Svelte, Express, Next.js, Nuxt
- **Go**: Gin, Echo, Fiber, Gorilla Mux
- **Python**: Django, Flask, FastAPI

**End-of-life tracking** for Node.js, Python, Go, Rust, and Java versions.

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

- More languages (C#, Kotlin, Scala, etc.)
- More frameworks
- Better version detection
- Updated EOL information
- New Ideas

## License

MIT License - see [LICENSE](./LICENSE) file for details.
