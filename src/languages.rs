use crate::project_info::LanguageStatus;

/// Get the end-of-life status for a language version
pub fn get_language_status(language: &str, version: &str) -> LanguageStatus {
    match language {
        "Rust" => get_rust_status(version),
        "JavaScript" => get_node_status(version),
        "Go" => get_go_status(version),
        "Python" => get_python_status(version),
        "Java" => get_java_status(version),
        _ => LanguageStatus::Unknown,
    }
}

fn get_rust_status(version_string: &str) -> LanguageStatus {
    let normalized_version = clean_version_string(version_string);

    match normalized_version.as_str() {
        version
            if version.starts_with("1.75")
                || version.starts_with("1.76")
                || version.starts_with("1.77")
                || version.starts_with("1.78") =>
        {
            LanguageStatus::Supported
        }
        version
            if version.starts_with("1.70")
                || version.starts_with("1.71")
                || version.starts_with("1.72")
                || version.starts_with("1.73")
                || version.starts_with("1.74") =>
        {
            LanguageStatus::EndingSoon {
                date: "2024-12-31".to_string(),
            }
        }
        version if version.starts_with("1.6") || version.starts_with("1.5") => {
            LanguageStatus::EndOfLife {
                date: "2023-01-01".to_string(),
            }
        }
        _ => LanguageStatus::Unknown,
    }
}

fn get_node_status(version_string: &str) -> LanguageStatus {
    let normalized_version = clean_version_string(version_string);
    let node_major_version = extract_major_version(&normalized_version);

    match node_major_version {
        20 => LanguageStatus::Supported,
        18 => LanguageStatus::Supported,
        16 => LanguageStatus::EndingSoon {
            date: "2024-04-30".to_string(),
        },
        14 => LanguageStatus::EndOfLife {
            date: "2023-04-30".to_string(),
        },
        12 => LanguageStatus::EndOfLife {
            date: "2022-04-30".to_string(),
        },
        version if version >= 21 => LanguageStatus::Supported,
        _ => LanguageStatus::EndOfLife {
            date: "2022-01-01".to_string(),
        },
    }
}

fn get_go_status(version_string: &str) -> LanguageStatus {
    let normalized_version = clean_version_string(version_string);

    match normalized_version.as_str() {
        version if version.starts_with("1.22") || version.starts_with("1.21") => {
            LanguageStatus::Supported
        }
        version if version.starts_with("1.20") => LanguageStatus::EndingSoon {
            date: "2024-08-01".to_string(),
        },
        version if version.starts_with("1.19") => LanguageStatus::EndOfLife {
            date: "2024-02-01".to_string(),
        },
        version if version.starts_with("1.18") => LanguageStatus::EndOfLife {
            date: "2023-08-01".to_string(),
        },
        _ => LanguageStatus::EndOfLife {
            date: "2023-01-01".to_string(),
        },
    }
}

fn get_python_status(version_string: &str) -> LanguageStatus {
    let normalized_version = clean_version_string(version_string);
    let version_segments: Vec<&str> = normalized_version.split('.').collect();

    if version_segments.len() >= 2 {
        let python_major = version_segments[0].parse::<u32>().unwrap_or(0);
        let python_minor = version_segments[1].parse::<u32>().unwrap_or(0);

        match (python_major, python_minor) {
            (3, 12) => LanguageStatus::Supported,
            (3, 11) => LanguageStatus::Supported,
            (3, 10) => LanguageStatus::Supported,
            (3, 9) => LanguageStatus::EndingSoon {
                date: "2025-10-01".to_string(),
            },
            (3, 8) => LanguageStatus::EndOfLife {
                date: "2024-10-01".to_string(),
            },
            (3, 7) => LanguageStatus::EndOfLife {
                date: "2023-06-27".to_string(),
            },
            (2, 7) => LanguageStatus::EndOfLife {
                date: "2020-01-01".to_string(),
            },
            (major, minor) if major >= 3 && minor >= 13 => LanguageStatus::Supported,
            _ => LanguageStatus::EndOfLife {
                date: "2023-01-01".to_string(),
            },
        }
    } else {
        LanguageStatus::Unknown
    }
}

fn get_java_status(version_string: &str) -> LanguageStatus {
    let normalized_version = clean_version_string(version_string);
    let java_major_version = extract_major_version(&normalized_version);

    match java_major_version {
        21 => LanguageStatus::Supported,
        17 => LanguageStatus::Supported,
        11 => LanguageStatus::Supported,
        8 => LanguageStatus::EndingSoon {
            date: "2025-12-31".to_string(),
        },
        version if version >= 22 => LanguageStatus::Supported,
        version if (9..=20).contains(&version) => LanguageStatus::EndOfLife {
            date: "2023-01-01".to_string(),
        },
        _ => LanguageStatus::EndOfLife {
            date: "2022-01-01".to_string(),
        },
    }
}

/// Clean version string by removing common prefixes and suffixes
fn clean_version_string(raw_version: &str) -> String {
    raw_version
        .trim()
        .trim_start_matches('v')
        .trim_start_matches(">=")
        .trim_start_matches("<=")
        .trim_start_matches('=')
        .trim_start_matches('>')
        .trim_start_matches('<')
        .trim_start_matches('^')
        .trim_start_matches('~')
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

/// Extract major version number from version string
fn extract_major_version(version_string: &str) -> u32 {
    version_string
        .split('.')
        .next()
        .and_then(|segment| segment.parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_version_string() {
        assert_eq!(clean_version_string("v1.70.0"), "1.70.0");
        assert_eq!(clean_version_string(">=3.11"), "3.11");
        assert_eq!(clean_version_string("^18.0.0"), "18.0.0");
        assert_eq!(clean_version_string("~2.7.0"), "2.7.0");
        assert_eq!(clean_version_string("1.21"), "1.21");
    }

    #[test]
    fn test_extract_major_version() {
        assert_eq!(extract_major_version("18.0.0"), 18);
        assert_eq!(extract_major_version("3.11.2"), 3);
        assert_eq!(extract_major_version("1.70"), 1);
        assert_eq!(extract_major_version("invalid"), 0);
    }

    #[test]
    fn test_rust_status() {
        assert_eq!(get_rust_status("1.75.0"), LanguageStatus::Supported);
        assert_eq!(
            get_rust_status("1.70.0"),
            LanguageStatus::EndingSoon {
                date: "2024-12-31".to_string()
            }
        );
        assert!(matches!(
            get_rust_status("1.60.0"),
            LanguageStatus::EndOfLife { .. }
        ));
    }

    #[test]
    fn test_node_status() {
        assert_eq!(get_node_status("20.0.0"), LanguageStatus::Supported);
        assert_eq!(get_node_status("18.0.0"), LanguageStatus::Supported);
        assert!(matches!(
            get_node_status("14.0.0"),
            LanguageStatus::EndOfLife { .. }
        ));
    }

    #[test]
    fn test_python_status() {
        assert_eq!(get_python_status("3.12.0"), LanguageStatus::Supported);
        assert_eq!(get_python_status("3.11.5"), LanguageStatus::Supported);
        assert!(matches!(
            get_python_status("3.8.0"),
            LanguageStatus::EndOfLife { .. }
        ));
        assert!(matches!(
            get_python_status("2.7.18"),
            LanguageStatus::EndOfLife { .. }
        ));
    }

    #[test]
    fn test_go_status() {
        assert_eq!(get_go_status("1.22.0"), LanguageStatus::Supported);
        assert_eq!(get_go_status("1.21.5"), LanguageStatus::Supported);
        assert!(matches!(
            get_go_status("1.19.0"),
            LanguageStatus::EndOfLife { .. }
        ));
    }

    #[test]
    fn test_java_status() {
        assert_eq!(get_java_status("21"), LanguageStatus::Supported);
        assert_eq!(get_java_status("17"), LanguageStatus::Supported);
        assert_eq!(get_java_status("11"), LanguageStatus::Supported);
        assert!(matches!(
            get_java_status("8"),
            LanguageStatus::EndingSoon { .. }
        ));
    }

    #[test]
    fn test_unknown_language() {
        assert_eq!(
            get_language_status("Unknown", "1.0.0"),
            LanguageStatus::Unknown
        );
    }
}
