use crate::project_info::FrameworkDetails;

/// Detailed information about a framework
pub fn get_framework_details(framework_name: &str) -> Option<FrameworkDetails> {
    match framework_name {
        "Axum" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Actix Web".to_string(),
                "Warp".to_string(),
                "Rocket".to_string(),
            ],
            is_popular: true,
            description: Some("Fast, ergonomic web framework built on Tokio".to_string()),
        }),
        "Actix Web" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec!["Axum".to_string(), "Warp".to_string(), "Rocket".to_string()],
            is_popular: true,
            description: Some("Powerful, pragmatic, and extremely fast web framework".to_string()),
        }),
        "Rocket" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Axum".to_string(),
                "Actix Web".to_string(),
                "Warp".to_string(),
            ],
            is_popular: true,
            description: Some("Type-safe, declarative web framework".to_string()),
        }),
        "Warp" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Axum".to_string(),
                "Actix Web".to_string(),
                "Rocket".to_string(),
            ],
            is_popular: false,
            description: Some("Composable web framework with filters".to_string()),
        }),
        "Bevy" => Some(FrameworkDetails {
            framework_type: "Game Engine".to_string(),
            alternatives: vec!["Amethyst".to_string(), "ggez".to_string()],
            is_popular: true,
            description: Some("Data-driven game engine built in Rust".to_string()),
        }),
        "Clap (CLI)" => Some(FrameworkDetails {
            framework_type: "CLI Framework".to_string(),
            alternatives: vec!["structopt".to_string(), "argh".to_string()],
            is_popular: true,
            description: Some("Command line argument parser".to_string()),
        }),
        "React" => Some(FrameworkDetails {
            framework_type: "Frontend Framework".to_string(),
            alternatives: vec![
                "Vue".to_string(),
                "Angular".to_string(),
                "Svelte".to_string(),
            ],
            is_popular: true,
            description: Some("JavaScript library for building user interfaces".to_string()),
        }),
        "Vue" => Some(FrameworkDetails {
            framework_type: "Frontend Framework".to_string(),
            alternatives: vec![
                "React".to_string(),
                "Angular".to_string(),
                "Svelte".to_string(),
            ],
            is_popular: true,
            description: Some("Progressive JavaScript framework for building UIs".to_string()),
        }),
        "Angular" => Some(FrameworkDetails {
            framework_type: "Frontend Framework".to_string(),
            alternatives: vec!["React".to_string(), "Vue".to_string(), "Svelte".to_string()],
            is_popular: true,
            description: Some(
                "Platform for building mobile and desktop web applications".to_string(),
            ),
        }),
        "Svelte" => Some(FrameworkDetails {
            framework_type: "Frontend Framework".to_string(),
            alternatives: vec![
                "React".to_string(),
                "Vue".to_string(),
                "Angular".to_string(),
            ],
            is_popular: true,
            description: Some("Compile-time framework that disappears into vanilla JS".to_string()),
        }),
        "Express" => Some(FrameworkDetails {
            framework_type: "Backend Framework".to_string(),
            alternatives: vec![
                "Fastify".to_string(),
                "Koa".to_string(),
                "NestJS".to_string(),
            ],
            is_popular: true,
            description: Some(
                "Fast, unopinionated, minimalist web framework for Node.js".to_string(),
            ),
        }),
        "Next.js" => Some(FrameworkDetails {
            framework_type: "Full-stack Framework".to_string(),
            alternatives: vec![
                "Nuxt".to_string(),
                "SvelteKit".to_string(),
                "Remix".to_string(),
            ],
            is_popular: true,
            description: Some(
                "React framework for production with hybrid static & server rendering".to_string(),
            ),
        }),
        "Nuxt" => Some(FrameworkDetails {
            framework_type: "Full-stack Framework".to_string(),
            alternatives: vec![
                "Next.js".to_string(),
                "SvelteKit".to_string(),
                "Remix".to_string(),
            ],
            is_popular: true,
            description: Some(
                "Intuitive Vue framework for creating universal applications".to_string(),
            ),
        }),
        "Gin" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Echo".to_string(),
                "Fiber".to_string(),
                "Gorilla Mux".to_string(),
            ],
            is_popular: true,
            description: Some("Fast HTTP web framework written in Go".to_string()),
        }),
        "Echo" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Gin".to_string(),
                "Fiber".to_string(),
                "Gorilla Mux".to_string(),
            ],
            is_popular: true,
            description: Some(
                "High performance, extensible, minimalist Go web framework".to_string(),
            ),
        }),
        "Fiber" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Gin".to_string(),
                "Echo".to_string(),
                "Gorilla Mux".to_string(),
            ],
            is_popular: true,
            description: Some("Express inspired web framework built on Fasthttp".to_string()),
        }),
        "Gorilla Mux" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec!["Gin".to_string(), "Echo".to_string(), "Fiber".to_string()],
            is_popular: false,
            description: Some(
                "Powerful HTTP router and URL matcher for building Go web servers".to_string(),
            ),
        }),
        "Django" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Flask".to_string(),
                "FastAPI".to_string(),
                "Pyramid".to_string(),
            ],
            is_popular: true,
            description: Some(
                "High-level Python web framework that encourages rapid development".to_string(),
            ),
        }),
        "Flask" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Django".to_string(),
                "FastAPI".to_string(),
                "Pyramid".to_string(),
            ],
            is_popular: true,
            description: Some("Lightweight WSGI web application framework".to_string()),
        }),
        "FastAPI" => Some(FrameworkDetails {
            framework_type: "Web Framework".to_string(),
            alternatives: vec![
                "Django".to_string(),
                "Flask".to_string(),
                "Starlette".to_string(),
            ],
            is_popular: true,
            description: Some(
                "Modern, fast web framework for building APIs with Python based on type hints"
                    .to_string(),
            ),
        }),
        _ => None,
    }
}

/// Get framework popularity ranking (1-10, where 10 is most popular)
///
/// This function provides a popularity score that could be useful for recommendations
/// or sorting frameworks by adoption.
#[allow(dead_code)]
pub fn get_framework_popularity(framework_name: &str) -> u8 {
    match framework_name {
        "React" | "Express" | "Django" | "Flask" => 10,
        "Vue" | "Angular" | "Next.js" | "FastAPI" | "Gin" => 9,
        "Svelte" | "Axum" | "Actix Web" | "Echo" | "Fiber" | "Nuxt" => 8,
        "Rocket" | "Bevy" | "Clap (CLI)" => 7,
        "Warp" | "Gorilla Mux" => 5,
        _ => 3,
    }
}

/// Check if a framework is considered enterprise-ready
///
// TODO: Need to check a framework's ecosystem, community support, and stability.
#[allow(dead_code)]
pub fn is_enterprise_ready(framework_name: &str) -> bool {
    matches!(
        framework_name,
        "React"
            | "Vue"
            | "Angular"
            | "Express"
            | "Next.js"
            | "Django"
            | "Flask"
            | "FastAPI"
            | "Gin"
            | "Echo"
            | "Actix Web"
            | "Axum"
    )
}

/// Get the learning difficulty (1-10, where 10 is most difficult)
///
// TODO: This should be based on community feedback, documentation quality, and complexity of the framework. Currently, it's a rough estimate.
#[allow(dead_code)]
pub fn get_learning_difficulty(framework_name: &str) -> u8 {
    match framework_name {
        "Express" | "Flask" | "Gin" => 2,
        "React" | "Vue" | "Echo" | "Axum" => 3,
        "Next.js" | "Svelte" | "FastAPI" | "Fiber" => 4,
        "Django" | "Actix Web" | "Rocket" => 5,
        "Nuxt" => 6,
        "Angular" => 8,
        "Bevy" => 7,
        _ => 5,
    }
}

/// Get common use cases for a framework
///
// TODO: Figure out a better way to determine use cases, possibly based on community projects or documentation.
#[allow(dead_code)]
pub fn get_use_cases(framework_name: &str) -> Vec<String> {
    match framework_name {
        "React" => vec![
            "Single Page Applications".to_string(),
            "Mobile Apps (React Native)".to_string(),
            "Desktop Apps (Electron)".to_string(),
            "Static Sites".to_string(),
        ],
        "Vue" => vec![
            "Progressive Web Apps".to_string(),
            "Single Page Applications".to_string(),
            "Static Sites".to_string(),
            "Mobile Apps".to_string(),
        ],
        "Angular" => vec![
            "Enterprise Applications".to_string(),
            "Single Page Applications".to_string(),
            "Progressive Web Apps".to_string(),
            "Desktop Apps".to_string(),
        ],
        "Express" => vec![
            "REST APIs".to_string(),
            "Web Applications".to_string(),
            "Microservices".to_string(),
            "Real-time Applications".to_string(),
        ],
        "Django" => vec![
            "Web Applications".to_string(),
            "REST APIs".to_string(),
            "CMS Systems".to_string(),
            "E-commerce Sites".to_string(),
        ],
        "FastAPI" => vec![
            "REST APIs".to_string(),
            "Microservices".to_string(),
            "Machine Learning APIs".to_string(),
            "Real-time Applications".to_string(),
        ],
        "Axum" => vec![
            "Web APIs".to_string(),
            "Microservices".to_string(),
            "High-performance Applications".to_string(),
            "System Services".to_string(),
        ],
        "Bevy" => vec![
            "2D Games".to_string(),
            "3D Games".to_string(),
            "Simulations".to_string(),
            "Interactive Applications".to_string(),
        ],
        _ => vec!["Web Development".to_string()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_framework_details() {
        let react_details = get_framework_details("React").unwrap();
        assert_eq!(react_details.framework_type, "Frontend Framework");
        assert!(react_details.is_popular);
        assert!(react_details.alternatives.contains(&"Vue".to_string()));

        let unknown_framework = get_framework_details("UnknownFramework");
        assert!(unknown_framework.is_none());
    }

    #[test]
    fn test_framework_popularity() {
        assert_eq!(get_framework_popularity("React"), 10);
        assert_eq!(get_framework_popularity("Vue"), 9);
        assert_eq!(get_framework_popularity("Warp"), 5);
        assert_eq!(get_framework_popularity("UnknownFramework"), 3);
    }

    #[test]
    fn test_is_enterprise_ready() {
        assert!(is_enterprise_ready("React"));
        assert!(is_enterprise_ready("Django"));
        assert!(is_enterprise_ready("Angular"));
        assert!(!is_enterprise_ready("Warp"));
        assert!(!is_enterprise_ready("UnknownFramework"));
    }

    #[test]
    fn test_learning_difficulty() {
        assert_eq!(get_learning_difficulty("Express"), 2);
        assert_eq!(get_learning_difficulty("React"), 3);
        assert_eq!(get_learning_difficulty("Angular"), 8);
        assert_eq!(get_learning_difficulty("UnknownFramework"), 5);
    }

    #[test]
    fn test_get_use_cases() {
        let react_use_cases = get_use_cases("React");
        assert!(react_use_cases.contains(&"Single Page Applications".to_string()));
        assert!(react_use_cases.len() > 1);

        let unknown_use_cases = get_use_cases("UnknownFramework");
        assert_eq!(unknown_use_cases, vec!["Web Development".to_string()]);
    }

    #[test]
    fn test_framework_details_completeness() {
        let major_frameworks = vec![
            "React",
            "Vue",
            "Angular",
            "Svelte",
            "Express",
            "Next.js",
            "Django",
            "Flask",
            "FastAPI",
            "Axum",
            "Actix Web",
            "Gin",
        ];

        for framework_name in major_frameworks {
            let framework_details = get_framework_details(framework_name);
            assert!(
                framework_details.is_some(),
                "Missing details for {}",
                framework_name
            );

            let details = framework_details.unwrap();
            assert!(!details.framework_type.is_empty());
            assert!(!details.alternatives.is_empty());
        }
    }
}
