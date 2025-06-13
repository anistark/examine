use examine::examine;

fn main() -> Result<(), String> {
    println!("🔍 Examine - Simple Project Analysis");
    println!("====================================\n");

    // Analyze current directory
    let info = examine(".")?;

    println!("📁 Project: {}", info.project_path);

    if let Some(ref name) = info.project_name {
        println!("📦 Name: {}", name);
    }

    println!("🔤 Language: {}", info.language);

    if let Some(ref version) = info.language_version {
        println!("📋 Version: {}", version);
        println!("⚡ Status: {}", info.language_status);
    } else {
        println!("📋 Version: Unknown");
        println!("⚡ Status: {}", info.language_status);
    }

    if let Some(ref framework) = info.framework {
        println!("🚀 Framework: {}", framework);

        if let Some(ref fw_version) = info.framework_version {
            println!("   📌 Version: {}", fw_version);
        }

        if let Some(ref details) = info.framework_details {
            println!("   🏷️  Type: {}", details.framework_type);
            println!(
                "   ⭐ Popular: {}",
                if details.is_popular { "Yes" } else { "No" }
            );

            if let Some(ref description) = details.description {
                println!("   📝 Description: {}", description);
            }

            if !details.alternatives.is_empty() {
                println!("   🔄 Alternatives: {}", details.alternatives.join(", "));
            }
        }
    } else {
        println!("🚀 Framework: None detected");
    }

    println!("\n✨ Summary: {}", info.summary());

    // Show status interpretation
    println!("\n💡 Status Guide:");
    println!("   ✅ Supported = Actively maintained, safe to use");
    println!("   ⚠️  Ending Soon = Will reach EOL within 6 months");
    println!("   ❌ End of Life = No longer supported, consider upgrading");
    println!("   ❓ Unknown = Status not tracked or version not detected");

    Ok(())
}
