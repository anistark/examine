use crate::frameworks;
use crate::languages;
use crate::project_info::ProjectInfo;
use std::fs;
use std::path::Path;

pub fn detect_project_info(project_path: &Path) -> Result<ProjectInfo, String> {
    if !project_path.exists() {
        return Err(format!("Path does not exist: {}", project_path.display()));
    }

    let detected_language = detect_language(project_path)?;
    let mut project_info = ProjectInfo::new(
        detected_language.clone(),
        project_path.display().to_string(),
    );

    if let Some(language_version) = detect_language_version(project_path, &detected_language) {
        let language_status = languages::get_language_status(&detected_language, &language_version);
        project_info = project_info
            .with_language_version(language_version)
            .with_language_status(language_status);
    }

    if let Some((framework_name, framework_version)) =
        detect_framework(project_path, &detected_language)
    {
        let framework_details = frameworks::get_framework_details(&framework_name);
        project_info = project_info.with_framework(framework_name, framework_version);
        if let Some(details) = framework_details {
            project_info = project_info.with_framework_details(details);
        }
    }

    if let Some(project_name) = detect_project_name(project_path, &detected_language) {
        project_info = project_info.with_project_name(project_name);
    }

    Ok(project_info)
}

fn detect_language(project_path: &Path) -> Result<String, String> {
    let language_detections = vec![
        ("Cargo.toml", "Rust"),
        ("package.json", "JavaScript"),
        ("go.mod", "Go"),
        ("pyproject.toml", "Python"),
        ("requirements.txt", "Python"),
        ("pom.xml", "Java"),
        ("build.gradle", "Java"),
        ("composer.json", "PHP"),
        ("Gemfile", "Ruby"),
        ("Package.swift", "Swift"),
        ("pubspec.yaml", "Dart"),
        ("mix.exs", "Elixir"),
        ("stack.yaml", "Haskell"),
        ("project.clj", "Clojure"),
        ("CMakeLists.txt", "C++"),
        ("Makefile", "C++"),
    ];

    for (project_file, detected_language) in language_detections {
        if project_path.join(project_file).exists() {
            return Ok(detected_language.to_string());
        }
    }

    if let Ok(directory_entries) = fs::read_dir(project_path) {
        let mut file_extension_counts = std::collections::HashMap::new();

        for directory_entry in directory_entries.flatten() {
            if let Some(file_extension) =
                directory_entry.path().extension().and_then(|e| e.to_str())
            {
                let detected_language = match file_extension {
                    "rs" => "Rust",
                    "js" | "ts" | "jsx" | "tsx" => "JavaScript",
                    "go" => "Go",
                    "py" => "Python",
                    "java" => "Java",
                    "php" => "PHP",
                    "rb" => "Ruby",
                    "swift" => "Swift",
                    "dart" => "Dart",
                    "ex" | "exs" => "Elixir",
                    "hs" => "Haskell",
                    "clj" | "cljs" => "Clojure",
                    "c" | "cpp" | "cc" | "cxx" => "C++",
                    "cs" => "C#",
                    _ => continue,
                };
                *file_extension_counts.entry(detected_language).or_insert(0) += 1;
            }
        }

        if let Some((most_common_language, _)) =
            file_extension_counts.iter().max_by_key(|(_, count)| *count)
        {
            return Ok(most_common_language.to_string());
        }
    }

    Err("Could not detect project language".to_string())
}

fn detect_language_version(project_path: &Path, detected_language: &str) -> Option<String> {
    match detected_language {
        "Rust" => detect_rust_version(project_path),
        "JavaScript" => detect_node_version(project_path),
        "Go" => detect_go_version(project_path),
        "Python" => detect_python_version(project_path),
        "Java" => detect_java_version(project_path),
        _ => None,
    }
}

fn detect_framework(
    project_path: &Path,
    detected_language: &str,
) -> Option<(String, Option<String>)> {
    match detected_language {
        "Rust" => detect_rust_framework(project_path),
        "JavaScript" => detect_javascript_framework(project_path),
        "Go" => detect_go_framework(project_path),
        "Python" => detect_python_framework(project_path),
        "Java" => detect_java_framework(project_path),
        _ => None,
    }
}

fn detect_project_name(project_path: &Path, detected_language: &str) -> Option<String> {
    match detected_language {
        "Rust" => {
            let cargo_toml_path = project_path.join("Cargo.toml");
            if let Ok(cargo_content) = fs::read_to_string(&cargo_toml_path) {
                if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&cargo_content) {
                    return parsed_toml
                        .get("package")
                        .and_then(|package_section| package_section.get("name"))
                        .and_then(|name_value| name_value.as_str())
                        .map(|name_string| name_string.to_string());
                }
            }
        }
        "JavaScript" => {
            let package_json_path = project_path.join("package.json");
            if let Ok(package_content) = fs::read_to_string(&package_json_path) {
                if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&package_content)
                {
                    return parsed_json
                        .get("name")
                        .and_then(|name_value| name_value.as_str())
                        .map(|name_string| name_string.to_string());
                }
            }
        }
        "Go" => {
            let go_mod_path = project_path.join("go.mod");
            if let Ok(go_mod_content) = fs::read_to_string(&go_mod_path) {
                for content_line in go_mod_content.lines() {
                    if content_line.starts_with("module ") {
                        let module_name = content_line.strip_prefix("module ").unwrap_or("").trim();
                        if let Some(last_path_segment) = module_name.split('/').last() {
                            return Some(last_path_segment.to_string());
                        }
                    }
                }
            }
        }
        "Python" => {
            let pyproject_path = project_path.join("pyproject.toml");
            if let Ok(pyproject_content) = fs::read_to_string(&pyproject_path) {
                if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&pyproject_content) {
                    return parsed_toml
                        .get("project")
                        .and_then(|project_section| project_section.get("name"))
                        .and_then(|name_value| name_value.as_str())
                        .map(|name_string| name_string.to_string());
                }
            }
        }
        _ => {}
    }

    project_path
        .file_name()
        .and_then(|directory_name| directory_name.to_str())
        .map(|directory_string| directory_string.to_string())
}

fn detect_rust_version(project_path: &Path) -> Option<String> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    if let Ok(cargo_content) = fs::read_to_string(&cargo_toml_path) {
        if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&cargo_content) {
            if let Some(rust_version) = parsed_toml
                .get("package")
                .and_then(|package_section| package_section.get("rust-version"))
                .and_then(|version_value| version_value.as_str())
            {
                return Some(rust_version.to_string());
            }
        }
    }

    let toolchain_toml_path = project_path.join("rust-toolchain.toml");
    if let Ok(toolchain_content) = fs::read_to_string(&toolchain_toml_path) {
        if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&toolchain_content) {
            if let Some(toolchain_channel) = parsed_toml
                .get("toolchain")
                .and_then(|toolchain_section| toolchain_section.get("channel"))
                .and_then(|channel_value| channel_value.as_str())
            {
                return Some(toolchain_channel.to_string());
            }
        }
    }

    let toolchain_file_path = project_path.join("rust-toolchain");
    if let Ok(toolchain_content) = fs::read_to_string(&toolchain_file_path) {
        return Some(toolchain_content.trim().to_string());
    }

    None
}

fn detect_node_version(project_path: &Path) -> Option<String> {
    let nvmrc_path = project_path.join(".nvmrc");
    if let Ok(nvmrc_content) = fs::read_to_string(&nvmrc_path) {
        return Some(nvmrc_content.trim().to_string());
    }

    let package_json_path = project_path.join("package.json");
    if let Ok(package_content) = fs::read_to_string(&package_json_path) {
        if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&package_content) {
            if let Some(node_version) = parsed_json
                .get("engines")
                .and_then(|engines_section| engines_section.get("node"))
                .and_then(|node_value| node_value.as_str())
            {
                return Some(node_version.to_string());
            }
        }
    }

    None
}

fn detect_go_version(project_path: &Path) -> Option<String> {
    let go_mod_path = project_path.join("go.mod");
    if let Ok(go_mod_content) = fs::read_to_string(&go_mod_path) {
        for content_line in go_mod_content.lines() {
            if content_line.starts_with("go ") {
                let go_version = content_line.strip_prefix("go ").unwrap_or("").trim();
                return Some(go_version.to_string());
            }
        }
    }
    None
}

fn detect_python_version(project_path: &Path) -> Option<String> {
    let python_version_path = project_path.join(".python-version");
    if let Ok(version_content) = fs::read_to_string(&python_version_path) {
        return Some(version_content.trim().to_string());
    }

    let pyproject_path = project_path.join("pyproject.toml");
    if let Ok(pyproject_content) = fs::read_to_string(&pyproject_path) {
        if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&pyproject_content) {
            if let Some(python_version) = parsed_toml
                .get("project")
                .and_then(|project_section| project_section.get("requires-python"))
                .and_then(|version_value| version_value.as_str())
            {
                return Some(python_version.to_string());
            }
        }
    }

    None
}

fn detect_java_version(_project_path: &Path) -> Option<String> {
    None
}

fn detect_rust_framework(project_path: &Path) -> Option<(String, Option<String>)> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    if let Ok(cargo_content) = fs::read_to_string(&cargo_toml_path) {
        if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&cargo_content) {
            if let Some(dependencies_table) = parsed_toml
                .get("dependencies")
                .and_then(|deps| deps.as_table())
            {
                if dependencies_table.contains_key("axum") {
                    let framework_version = get_dependency_version(dependencies_table, "axum");
                    return Some(("Axum".to_string(), framework_version));
                }
                if dependencies_table.contains_key("actix-web") {
                    let framework_version = get_dependency_version(dependencies_table, "actix-web");
                    return Some(("Actix Web".to_string(), framework_version));
                }
                if dependencies_table.contains_key("warp") {
                    let framework_version = get_dependency_version(dependencies_table, "warp");
                    return Some(("Warp".to_string(), framework_version));
                }
                if dependencies_table.contains_key("rocket") {
                    let framework_version = get_dependency_version(dependencies_table, "rocket");
                    return Some(("Rocket".to_string(), framework_version));
                }
                if dependencies_table.contains_key("clap") {
                    let framework_version = get_dependency_version(dependencies_table, "clap");
                    return Some(("Clap (CLI)".to_string(), framework_version));
                }
                if dependencies_table.contains_key("bevy") {
                    let framework_version = get_dependency_version(dependencies_table, "bevy");
                    return Some(("Bevy".to_string(), framework_version));
                }
            }
        }
    }
    None
}

fn detect_javascript_framework(project_path: &Path) -> Option<(String, Option<String>)> {
    let package_json_path = project_path.join("package.json");
    if let Ok(package_content) = fs::read_to_string(&package_json_path) {
        if let Ok(parsed_json) = serde_json::from_str::<serde_json::Value>(&package_content) {
            let production_dependencies = parsed_json
                .get("dependencies")
                .and_then(|deps| deps.as_object());
            let development_dependencies = parsed_json
                .get("devDependencies")
                .and_then(|deps| deps.as_object());

            for dependency_object in [production_dependencies, development_dependencies]
                .iter()
                .filter_map(|deps| *deps)
            {
                if dependency_object.contains_key("react") {
                    let framework_version = dependency_object
                        .get("react")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("React".to_string(), framework_version));
                }
                if dependency_object.contains_key("vue") {
                    let framework_version = dependency_object
                        .get("vue")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Vue".to_string(), framework_version));
                }
                if dependency_object.contains_key("@angular/core") {
                    let framework_version = dependency_object
                        .get("@angular/core")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Angular".to_string(), framework_version));
                }
                if dependency_object.contains_key("svelte") {
                    let framework_version = dependency_object
                        .get("svelte")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Svelte".to_string(), framework_version));
                }
                if dependency_object.contains_key("express") {
                    let framework_version = dependency_object
                        .get("express")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Express".to_string(), framework_version));
                }
                if dependency_object.contains_key("next") {
                    let framework_version = dependency_object
                        .get("next")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Next.js".to_string(), framework_version));
                }
                if dependency_object.contains_key("nuxt") {
                    let framework_version = dependency_object
                        .get("nuxt")
                        .and_then(|version| version.as_str())
                        .map(|s| s.to_string());
                    return Some(("Nuxt".to_string(), framework_version));
                }
            }
        }
    }
    None
}

fn detect_go_framework(project_path: &Path) -> Option<(String, Option<String>)> {
    let go_mod_path = project_path.join("go.mod");
    if let Ok(go_mod_content) = fs::read_to_string(&go_mod_path) {
        if go_mod_content.contains("github.com/gin-gonic/gin") {
            return Some(("Gin".to_string(), None));
        }
        if go_mod_content.contains("github.com/gorilla/mux") {
            return Some(("Gorilla Mux".to_string(), None));
        }
        if go_mod_content.contains("github.com/labstack/echo") {
            return Some(("Echo".to_string(), None));
        }
        if go_mod_content.contains("github.com/gofiber/fiber") {
            return Some(("Fiber".to_string(), None));
        }
    }
    None
}

fn detect_python_framework(project_path: &Path) -> Option<(String, Option<String>)> {
    let requirements_path = project_path.join("requirements.txt");
    if let Ok(requirements_content) = fs::read_to_string(&requirements_path) {
        for requirement_line in requirements_content.lines() {
            let normalized_line = requirement_line.trim().to_lowercase();
            if normalized_line.starts_with("django") {
                return Some((
                    "Django".to_string(),
                    extract_version_from_requirement(&normalized_line),
                ));
            }
            if normalized_line.starts_with("flask") {
                return Some((
                    "Flask".to_string(),
                    extract_version_from_requirement(&normalized_line),
                ));
            }
            if normalized_line.starts_with("fastapi") {
                return Some((
                    "FastAPI".to_string(),
                    extract_version_from_requirement(&normalized_line),
                ));
            }
        }
    }

    let pyproject_path = project_path.join("pyproject.toml");
    if let Ok(pyproject_content) = fs::read_to_string(&pyproject_path) {
        if let Ok(parsed_toml) = toml::from_str::<toml::Value>(&pyproject_content) {
            if let Some(dependency_array) = parsed_toml
                .get("project")
                .and_then(|project_section| project_section.get("dependencies"))
                .and_then(|deps| deps.as_array())
            {
                for dependency_value in dependency_array {
                    if let Some(dependency_string) = dependency_value.as_str() {
                        let normalized_dependency = dependency_string.to_lowercase();
                        if normalized_dependency.starts_with("django") {
                            return Some((
                                "Django".to_string(),
                                extract_version_from_requirement(&normalized_dependency),
                            ));
                        }
                        if normalized_dependency.starts_with("flask") {
                            return Some((
                                "Flask".to_string(),
                                extract_version_from_requirement(&normalized_dependency),
                            ));
                        }
                        if normalized_dependency.starts_with("fastapi") {
                            return Some((
                                "FastAPI".to_string(),
                                extract_version_from_requirement(&normalized_dependency),
                            ));
                        }
                    }
                }
            }
        }
    }

    None
}

fn detect_java_framework(_project_path: &Path) -> Option<(String, Option<String>)> {
    None
}

fn get_dependency_version(
    dependencies_table: &toml::map::Map<String, toml::Value>,
    dependency_name: &str,
) -> Option<String> {
    dependencies_table
        .get(dependency_name)
        .and_then(|dependency_value| match dependency_value {
            toml::Value::String(version_string) => Some(version_string.clone()),
            toml::Value::Table(dependency_table) => dependency_table
                .get("version")
                .and_then(|version_value| version_value.as_str())
                .map(|version_string| version_string.to_string()),
            _ => None,
        })
}

fn extract_version_from_requirement(requirement: &str) -> Option<String> {
    // Extract version from requirement strings like "django==4.2.0" or "flask>=2.0.0"
    let re = regex::Regex::new(r"[><=!]+(.+)").ok()?;
    if let Some(captures) = re.captures(requirement) {
        return captures.get(1).map(|m| m.as_str().to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_detect_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"[package]
name = "test-project"
version = "0.1.0"
rust-version = "1.70"

[dependencies]
axum = "0.7"
"#,
        )
        .unwrap();

        let info = detect_project_info(temp_dir.path()).unwrap();
        assert_eq!(info.language, "Rust");
        assert_eq!(info.project_name, Some("test-project".to_string()));
        assert_eq!(info.language_version, Some("1.70".to_string()));
        assert_eq!(info.framework, Some("Axum".to_string()));
    }

    #[test]
    fn test_detect_javascript_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{
  "name": "my-react-app",
  "version": "1.0.0",
  "engines": {
    "node": "18.0.0"
  },
  "dependencies": {
    "react": "^18.2.0"
  }
}"#,
        )
        .unwrap();

        let info = detect_project_info(temp_dir.path()).unwrap();
        assert_eq!(info.language, "JavaScript");
        assert_eq!(info.project_name, Some("my-react-app".to_string()));
        assert_eq!(info.language_version, Some("18.0.0".to_string()));
        assert_eq!(info.framework, Some("React".to_string()));
    }

    #[test]
    fn test_detect_go_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("go.mod"),
            r#"module github.com/example/my-go-app

go 1.21

require github.com/gin-gonic/gin v1.9.0
"#,
        )
        .unwrap();

        let info = detect_project_info(temp_dir.path()).unwrap();
        assert_eq!(info.language, "Go");
        assert_eq!(info.project_name, Some("my-go-app".to_string()));
        assert_eq!(info.language_version, Some("1.21".to_string()));
        assert_eq!(info.framework, Some("Gin".to_string()));
    }

    #[test]
    fn test_detect_python_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("pyproject.toml"),
            r#"[project]
name = "my-django-app"
requires-python = ">=3.11"
dependencies = [
    "django>=4.2.0"
]
"#,
        )
        .unwrap();

        let info = detect_project_info(temp_dir.path()).unwrap();
        assert_eq!(info.language, "Python");
        assert_eq!(info.project_name, Some("my-django-app".to_string()));
        assert_eq!(info.language_version, Some(">=3.11".to_string()));
        assert_eq!(info.framework, Some("Django".to_string()));
    }

    #[test]
    fn test_detect_language_by_files() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(temp_dir.path().join("lib.rs"), "pub fn hello() {}").unwrap();

        let language = detect_language(temp_dir.path()).unwrap();
        assert_eq!(language, "Rust");
    }

    #[test]
    fn test_nonexistent_path() {
        let result = detect_project_info(Path::new("/definitely/does/not/exist"));
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_version_from_requirement() {
        assert_eq!(
            extract_version_from_requirement("django==4.2.0"),
            Some("4.2.0".to_string())
        );
        assert_eq!(
            extract_version_from_requirement("flask>=2.0.0"),
            Some("2.0.0".to_string())
        );
        assert_eq!(
            extract_version_from_requirement("fastapi~=0.68.0"),
            Some("0.68.0".to_string())
        );
        assert_eq!(extract_version_from_requirement("requests"), None);
    }
}
