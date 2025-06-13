use examine::examine;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "analyze" => {
            let project_path = args.get(2).unwrap_or(&".".to_string()).clone();
            if let Err(error_message) = analyze_project(&project_path) {
                eprintln!("Error: {}", error_message);
                process::exit(1);
            }
        }
        "help" | "--help" | "-h" => print_usage(&args[0]),
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    println!("Examine CLI - Project Analysis Tool");
    println!("Usage: {} analyze [PATH]", program_name);
    println!("       {} help", program_name);
    println!();
    println!("Examples:");
    println!("  {} analyze .", program_name);
    println!("  {} analyze /path/to/project", program_name);
}

fn analyze_project(project_path: &str) -> Result<(), String> {
    println!("Analyzing project at: {}", project_path);
    println!();

    let project_info = examine(project_path)?;

    println!("📁 Project: {}", project_info.project_path);

    if let Some(ref project_name) = project_info.project_name {
        println!("📦 Name: {}", project_name);
    }

    println!("🔤 Language: {}", project_info.language);

    if let Some(ref language_version) = project_info.language_version {
        println!("📋 Version: {}", language_version);
        println!("⚡ Status: {}", project_info.language_status);
    } else {
        println!("📋 Version: Unknown");
        println!("⚡ Status: {}", project_info.language_status);
    }

    if let Some(ref framework_name) = project_info.framework {
        println!("🚀 Framework: {}", framework_name);

        if let Some(ref framework_version) = project_info.framework_version {
            println!("   Version: {}", framework_version);
        }

        if let Some(ref framework_details) = project_info.framework_details {
            println!("   Type: {}", framework_details.framework_type);
            println!(
                "   Popular: {}",
                if framework_details.is_popular {
                    "Yes"
                } else {
                    "No"
                }
            );

            if let Some(ref framework_description) = framework_details.description {
                println!("   Description: {}", framework_description);
            }

            if !framework_details.alternatives.is_empty() {
                println!(
                    "   Alternatives: {}",
                    framework_details.alternatives.join(", ")
                );
            }
        }
    } else {
        println!("🚀 Framework: None detected");
    }

    println!();
    println!("✨ Summary: {}", project_info.summary());

    Ok(())
}
