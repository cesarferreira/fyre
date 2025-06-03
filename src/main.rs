use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, ContentArrangement, Table};
use regex::Regex;
use skim::prelude::*;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use std::process::{Command, exit};

#[derive(Parser)]
#[command(name = "fyre")]
#[command(about = "Flutter development automation tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate code or assets
    Generate {
        #[command(subcommand)]
        generate_type: GenerateType,
    },
    /// Alias for generate (shorthand)
    Gen,
    /// Release the app to different tracks
    Release {
        /// Release track (beta, production)
        track: String,
    },
    /// Deep clean and rebuild the project
    Clean,
    /// Open external services
    Open {
        /// Service to open (apple, android)
        service: String,
    },
    /// Apply automatic fixes to Dart code
    Fix,
    /// Bump version numbers
    Bump {
        /// Version component to bump (major, minor, patch, build)
        component: String,
    },
    /// Watch for changes and rebuild
    Watch,
    /// Fuzzy search and execute commands
    Search,
}

#[derive(Subcommand)]
enum GenerateType {
    /// Generate Swagger/OpenAPI client
    Swagger,
    /// Generate app icons
    Icon,
    /// Generate asset definitions
    Assets,
}

struct Utils;

impl Utils {
    fn is_flutter_project() -> bool {
        Path::new("pubspec.yaml").exists()
    }

    fn interrupt_if_non_flutter_project() -> Result<()> {
        if !Self::is_flutter_project() {
            eprintln!("{}", "This is not a flutter project...".red());
            exit(1);
        }
        Ok(())
    }

    fn execute(command: &str) -> Result<()> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let mut cmd = Command::new(parts[0]);
        if parts.len() > 1 {
            cmd.args(&parts[1..]);
        }

        let status = cmd.status().context(format!("Failed to execute command: {}", command))?;

        if !status.success() {
            println!("\n\n======================================================\n");
            println!(" {}", "Something went wrong while executing this:".red());
            println!("  {} {}\n", "$".yellow(), command.yellow());
            println!("======================================================\n");
            exit(1);
        }

        Ok(())
    }

    fn run_fastlane(lane: &str) -> Result<()> {
        Self::interrupt_if_non_flutter_project()?;
        
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir("./ios/")?;
        
        Self::execute("pod install")?;
        Self::execute(&format!("fastlane {}", lane))?;
        
        std::env::set_current_dir(original_dir)?;
        Ok(())
    }
}

struct VersionBumper;

impl VersionBumper {
    fn bump_version(component: &str) -> Result<()> {
        let pubspec_content = fs::read_to_string("pubspec.yaml")
            .context("Failed to read pubspec.yaml")?;

        let version_re = Regex::new(r"version: (\d+)\.(\d+)\.(\d+)\+(\d+)")
            .context("Failed to compile version regex")?;

        let version_match = version_re.find(&pubspec_content)
            .context("Could not find version line in pubspec.yaml")?;

        let captures = version_re.captures(&pubspec_content)
            .context("Failed to capture version components")?;

        let major: u32 = captures[1].parse()?;
        let minor: u32 = captures[2].parse()?;
        let patch: u32 = captures[3].parse()?;
        let build: u32 = captures[4].parse()?;

        let old_version = format!("{}.{}.{}+{}", major, minor, patch, build);

        let (new_major, new_minor, new_patch, new_build) = match component {
            "major" => (major + 1, 0, 0, build + 1),
            "minor" => (major, minor + 1, 0, build + 1),
            "patch" => (major, minor, patch + 1, build + 1),
            "build" => (major, minor, patch, build + 1),
            _ => {
                eprintln!("Invalid update type: {}", component);
                eprintln!("Please use 'major', 'minor', 'patch', or 'build'");
                exit(1);
            }
        };

        let new_version = format!("{}.{}.{}+{}", new_major, new_minor, new_patch, new_build);
        let new_content = pubspec_content.replace(&version_match.as_str(), &format!("version: {}", new_version));

        fs::write("pubspec.yaml", new_content)
            .context("Failed to write updated pubspec.yaml")?;

        println!("\nfrom: {}", old_version.yellow());
        println!("to:   {}", new_version.green());

        Ok(())
    }
}

fn show_help() {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Command").add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Description").add_attribute(comfy_table::Attribute::Bold),
        ]);

    let commands = vec![
        ("f clean", "Deep cleans the project and rebuilds it."),
        ("", ""),
        ("f fix", "Automatically identifies and corrects common issues in Dart code, such as outdated syntax"),
        ("", ""),
        ("f generate swagger", "Executes a function to generate a Swagger (OpenAPI) client."),
        ("f generate icon", "Generates the icons for the app."),
        ("f generate assets", "Initiates asset generation and management. (using \"fluttergen\")"),
        ("", ""),
        ("f watch", "watch build_runner for code changes."),
        ("", ""),
        ("f open apple", "Opens the appstoreconnect website."),
        ("f open android", "Opens the play console website."),
        ("", ""),
        ("f release beta", "Releases the current build to the beta track."),
        ("f release production", "Releases the current build to the production track."),
        ("", ""),
        ("f bump major", "bumps the MAJOR build number (x.0.0+y)"),
        ("f bump minor", "bumps the MINOR build number (0.x.0+y)"),
        ("f bump patch", "bumps the PATCH build number (0.0.x+y)"),
        ("f bump build", "bumps the build number (1.0.0+y)"),
        ("", ""),
        ("f search", "Fuzzy search and execute commands interactively."),
        ("f help", "Shows a table with all the available commands."),
    ];

    for (command, description) in commands {
        if command.is_empty() {
            table.add_row(vec!["", ""]);
        } else {
            table.add_row(vec![
                Cell::new(command).fg(comfy_table::Color::Green),
                Cell::new(description),
            ]);
        }
    }

    println!("{}", table);
}

fn fuzzy_search() -> Result<()> {
    let commands = vec![
        "clean - Deep cleans the project and rebuilds it",
        "fix - Apply automatic fixes to Dart code",
        "generate swagger - Generate Swagger/OpenAPI client",
        "generate icon - Generate app icons",
        "generate assets - Generate asset definitions",
        "watch - Watch for changes and rebuild",
        "open apple - Open App Store Connect",
        "open android - Open Google Play Console",
        "release beta - Release to beta track",
        "release production - Release to production track",
        "bump major - Bump major version",
        "bump minor - Bump minor version",
        "bump patch - Bump patch version",
        "bump build - Bump build number",
        "help - Show help information",
    ];

    let input = commands.join("\n");
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(false)
        .prompt(Some("Select command: "))
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    if let Some(output) = Skim::run_with(&options, Some(items)) {
        if !output.is_abort {
            if let Some(selected_item) = output.selected_items.first() {
                let selected = selected_item.output().to_string();
                let command_part = selected.split(" - ").next().unwrap_or("");
                
                println!("Executing: {}", command_part.green());
                
                // Parse and execute the selected command
                let parts: Vec<&str> = command_part.split_whitespace().collect();
                match parts.as_slice() {
                    ["clean"] => handle_clean()?,
                    ["fix"] => handle_fix()?,
                    ["generate", "swagger"] => handle_generate_swagger()?,
                    ["generate", "icon"] => handle_generate_icon()?,
                    ["generate", "assets"] => handle_generate_assets()?,
                    ["watch"] => handle_watch()?,
                    ["open", service] => handle_open(service)?,
                    ["release", track] => handle_release(track)?,
                    ["bump", component] => handle_bump(component)?,
                    ["help"] => show_help(),
                    _ => println!("Command not implemented in fuzzy search"),
                }
            }
        }
    }

    Ok(())
}

fn handle_generate_swagger() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    Utils::execute("dart run build_runner build --delete-conflicting-outputs")
}

fn handle_generate_icon() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    Utils::execute("dart run flutter_launcher_icons")
}

fn handle_generate_assets() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    Utils::execute("fluttergen -c pubspec.yaml")
}

fn handle_release(track: &str) -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    
    match track {
        "beta" => Utils::run_fastlane("ios beta"),
        "production" | "release" => Utils::run_fastlane("ios release"),
        _ => {
            println!("Not ready for {} yet", track.red());
            Ok(())
        }
    }
}

fn handle_clean() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    
    Utils::execute("flutter clean")?;
    Utils::execute("flutter pub get")?;

    if Path::new("./ios/").exists() {
        let original_dir = std::env::current_dir()?;
        std::env::set_current_dir("./ios/")?;
        
        // Remove Podfile.lock if it exists
        if Path::new("Podfile.lock").exists() {
            fs::remove_file("Podfile.lock")?;
        }
        
        if Path::new("Podfile").exists() {
            Utils::execute("pod install")?;
        } else {
            println!("Podfile not found. Skipping pod install.");
        }
        
        std::env::set_current_dir(original_dir)?;
    }

    Ok(())
}

fn handle_open(service: &str) -> Result<()> {
    let url = match service {
        "apple" | "ios" => "https://appstoreconnect.apple.com/apps",
        "google" | "android" => "https://play.google.com/console/u/0/developers/",
        _ => {
            println!("Don't recognise the service: {}", service);
            return Ok(());
        }
    };

    opener::open(url).context("Failed to open URL")?;
    Ok(())
}

fn handle_fix() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    Utils::execute("dart fix --apply")
}

fn handle_bump(component: &str) -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    VersionBumper::bump_version(component)
}

fn handle_watch() -> Result<()> {
    Utils::interrupt_if_non_flutter_project()?;
    Utils::execute("dart run build_runner watch --delete-conflicting-outputs")
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Generate { generate_type }) => {
            match generate_type {
                GenerateType::Swagger => handle_generate_swagger()?,
                GenerateType::Icon => handle_generate_icon()?,
                GenerateType::Assets => handle_generate_assets()?,
            }
        }
        Some(Commands::Gen) => {
            // For now, Gen defaults to swagger generation like in the Ruby version
            handle_generate_swagger()?;
        }
        Some(Commands::Release { track }) => handle_release(&track)?,
        Some(Commands::Clean) => handle_clean()?,
        Some(Commands::Open { service }) => handle_open(&service)?,
        Some(Commands::Fix) => handle_fix()?,
        Some(Commands::Bump { component }) => handle_bump(&component)?,
        Some(Commands::Watch) => handle_watch()?,
        Some(Commands::Search) => fuzzy_search()?,
        None => {
            // Show clap help when no command is provided
            let mut cmd = Cli::command();
            cmd.print_help()?;
            println!(); // Add a newline for better formatting
        }
    }

    Ok(())
}
