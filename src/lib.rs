mod detector;
mod frameworks;
mod languages;
mod project_info;

pub use project_info::{LanguageStatus, ProjectInfo};

/// Examines the project at the given path and returns its information.
pub fn examine<P: AsRef<std::path::Path>>(path: P) -> Result<ProjectInfo, String> {
    detector::detect_project_info(path.as_ref())
}
