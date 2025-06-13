use serde::{Deserialize, Serialize};
use std::fmt;

/// Complete information about a detected project
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// End of life status for language versions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LanguageStatus {
    Supported,
    EndingSoon { date: String },
    EndOfLife { date: String },
    Unknown,
}

/// Detailed information about the detected framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkDetails {
    pub framework_type: String,
    pub alternatives: Vec<String>,
    pub is_popular: bool,
    pub description: Option<String>,
}

impl fmt::Display for LanguageStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LanguageStatus::Supported => write!(formatter, "✅ Supported"),
            LanguageStatus::EndingSoon { date } => write!(formatter, "⚠️ Ending Soon ({})", date),
            LanguageStatus::EndOfLife { date } => write!(formatter, "❌ End of Life ({})", date),
            LanguageStatus::Unknown => write!(formatter, "❓ Unknown"),
        }
    }
}

impl ProjectInfo {
    /// Create a new project info instance
    pub fn new<P: Into<String>>(detected_language: String, analyzed_project_path: P) -> Self {
        Self {
            language: detected_language,
            language_version: None,
            language_status: LanguageStatus::Unknown,
            framework: None,
            framework_version: None,
            framework_details: None,
            project_name: None,
            project_path: analyzed_project_path.into(),
        }
    }

    /// Set the language version
    pub fn with_language_version(mut self, detected_version: String) -> Self {
        self.language_version = Some(detected_version);
        self
    }

    /// Set the language status
    pub fn with_language_status(mut self, eol_status: LanguageStatus) -> Self {
        self.language_status = eol_status;
        self
    }

    /// Set the framework
    pub fn with_framework(
        mut self,
        detected_framework: String,
        detected_framework_version: Option<String>,
    ) -> Self {
        self.framework = Some(detected_framework);
        self.framework_version = detected_framework_version;
        self
    }

    /// Set framework details
    pub fn with_framework_details(mut self, additional_details: FrameworkDetails) -> Self {
        self.framework_details = Some(additional_details);
        self
    }

    /// Set the project name
    pub fn with_project_name(mut self, detected_name: String) -> Self {
        self.project_name = Some(detected_name);
        self
    }

    /// Get a summary string of the project
    pub fn summary(&self) -> String {
        let mut summary_parts = vec![self.language.clone()];

        if let Some(ref version) = self.language_version {
            summary_parts.push(format!("v{}", version));
        }

        if let Some(ref framework_name) = self.framework {
            if let Some(ref framework_version) = self.framework_version {
                summary_parts.push(format!("{} v{}", framework_name, framework_version));
            } else {
                summary_parts.push(framework_name.clone());
            }
        }

        summary_parts.join(" + ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_info_creation() {
        let project_info = ProjectInfo::new("Rust".to_string(), "/test/path");
        assert_eq!(project_info.language, "Rust");
        assert_eq!(project_info.project_path, "/test/path");
        assert!(project_info.language_version.is_none());
        assert_eq!(project_info.language_status, LanguageStatus::Unknown);
    }

    #[test]
    fn test_project_info_builder() {
        let project_info = ProjectInfo::new("Rust".to_string(), "/test/path")
            .with_language_version("1.70.0".to_string())
            .with_language_status(LanguageStatus::Supported)
            .with_framework("Axum".to_string(), Some("0.7.0".to_string()))
            .with_project_name("my-app".to_string());

        assert_eq!(project_info.language_version, Some("1.70.0".to_string()));
        assert_eq!(project_info.language_status, LanguageStatus::Supported);
        assert_eq!(project_info.framework, Some("Axum".to_string()));
        assert_eq!(project_info.framework_version, Some("0.7.0".to_string()));
        assert_eq!(project_info.project_name, Some("my-app".to_string()));
    }

    #[test]
    fn test_summary() {
        let project_info = ProjectInfo::new("JavaScript".to_string(), "/test")
            .with_language_version("18.0".to_string())
            .with_framework("React".to_string(), Some("18.2.0".to_string()));

        assert_eq!(project_info.summary(), "JavaScript + v18.0 + React v18.2.0");
    }

    #[test]
    fn test_language_status_display() {
        assert_eq!(LanguageStatus::Supported.to_string(), "✅ Supported");
        assert_eq!(
            LanguageStatus::EndingSoon {
                date: "2024-12-31".to_string()
            }
            .to_string(),
            "⚠️ Ending Soon (2024-12-31)"
        );
        assert_eq!(
            LanguageStatus::EndOfLife {
                date: "2023-04-01".to_string()
            }
            .to_string(),
            "❌ End of Life (2023-04-01)"
        );
        assert_eq!(LanguageStatus::Unknown.to_string(), "❓ Unknown");
    }

    #[test]
    fn test_framework_details() {
        let framework_details = FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec!["Express".to_string(), "Fastify".to_string()],
            is_popular: true,
            description: Some("Fast web framework".to_string()),
        };

        assert_eq!(framework_details.framework_type, "Web Framework");
        assert_eq!(framework_details.alternatives.len(), 2);
        assert!(framework_details.is_popular);
        assert!(framework_details.description.is_some());
    }
}
