mod catalog;
mod font;
mod local_data;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use indicatif::MultiProgress;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Mutex;
use toml_edit::{Array, DocumentMut, Item, Table, Value, value};

use crate::catalog::Catalog;
use crate::font::{FontInfo, FontStatus};
use crate::local_data::LocalData;

#[derive(Parser, Debug)]
#[command(name = "kateb")]
#[command(about = "A font manager for Persian (Farsi) fonts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a new font
    Install {
        #[arg(trailing_var_arg = true)]
        fonts: Vec<String>,
    },
    /// Update an installed font
    Update {
        #[arg(trailing_var_arg = true)]
        fonts: Vec<String>,
    },
    /// Reinstall an already-installed font
    Reinstall {
        #[arg(trailing_var_arg = true)]
        fonts: Vec<String>,
    },
    /// Uninstall an already-installed font
    Uninstall {
        #[arg(trailing_var_arg = true)]
        fonts: Vec<String>,
    },
    /// List all supported Farsi fonts
    List,
    /// Show the fonts that are currently installed
    Fonts,
    /// Display brief information about a font's publisher
    Info {
        #[arg(trailing_var_arg = true)]
        fonts: Vec<String>,
    },
    /// Display the kateb version
    #[command(alias = "v", visible_alias = "-v")]
    Version,
    /// Upgrade the kateb tool itself
    SelfUpgrade,
}

fn run(command: Commands) -> Result<(), ()> {
    match command {
        Commands::Install { fonts } => {
            let results = install(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?;
            print_results_with_info("installed", &results);
        }
        Commands::Update { fonts } => {
            let results = update(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?;
            print_results_with_info("updated", &results);
        }
        Commands::Reinstall { fonts } => {
            let results = reinstall(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?;
            print_results_with_info("reinstalled", &results);
        }
        Commands::Uninstall { fonts } => {
            let results = uninstall(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?;
            print_results_simple("uninstalled", &results);
        }
        Commands::Info { fonts } => info(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::List => show_supported_fonts(),
        Commands::Fonts => list_installed_fonts().map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::Version => println!("Version: {}", env!("CARGO_PKG_VERSION")),
        Commands::SelfUpgrade => println!("self‐upgrade..."),
    }
    Ok(())
}

fn print_results_with_info(action: &str, results: &[(String, FontStatus, FontInfo)]) {
    let errors: Vec<_> = results
        .iter()
        .filter(|(_, status, _)| matches!(status, FontStatus::Error(_)))
        .collect();
    let warnings: Vec<_> = results
        .iter()
        .filter(|(_, status, _)| matches!(status, FontStatus::Warning(_)))
        .collect();
    let successes: Vec<_> = results
        .iter()
        .filter(|(_, status, _)| matches!(status, FontStatus::Success))
        .collect();

    if !errors.is_empty() {
        eprintln!("❌ Errors:");
        for (name, status, _) in &errors {
            if let FontStatus::Error(msg) = status {
                eprintln!("  {} - {}", name, msg);
            }
        }
    }

    if !warnings.is_empty() {
        eprintln!("⚠ :");
        for (name, status, _) in &warnings {
            if let FontStatus::Warning(msg) = status {
                eprintln!("  {} - {}", name, msg);
            }
        }
    }

    if !successes.is_empty() {
        println!("✅ {}:", action);
        for (name, _, info) in &successes {
            println!("  {} {} - {}", name, info.tag_name, info.update_date);
        }
    }

    if results.is_empty() || (errors.is_empty() && warnings.is_empty() && successes.is_empty()) {
        println!("nothing to {action}");
    }
}

fn print_results_simple(action: &str, results: &[(String, FontStatus)]) {
    let errors: Vec<_> = results
        .iter()
        .filter(|(_, status)| matches!(status, FontStatus::Error(_)))
        .collect();
    let warnings: Vec<_> = results
        .iter()
        .filter(|(_, status)| matches!(status, FontStatus::Warning(_)))
        .collect();
    let successes: Vec<_> = results
        .iter()
        .filter(|(_, status)| matches!(status, FontStatus::Success))
        .collect();

    if !errors.is_empty() {
        eprintln!("❌ Errors:");
        for (name, status) in &errors {
            if let FontStatus::Error(msg) = status {
                eprintln!("  {} - {}", name, msg);
            }
        }
    }

    if !warnings.is_empty() {
        eprintln!("⚠ :");
        for (name, status) in &warnings {
            if let FontStatus::Warning(msg) = status {
                eprintln!("  {} - {}", name, msg);
            }
        }
    }

    if !successes.is_empty() {
        println!("✅ {}:", action);
        for (name, _) in &successes {
            println!("  {}", name);
        }
    }

    if results.is_empty() || (errors.is_empty() && warnings.is_empty() && successes.is_empty()) {
        println!("nothing to {action}");
    }
}

fn save_font_info(info: &FontInfo) -> Result<()> {
    let mut local_data = LocalData::new();
    let table = font_toml_table(&info.tag_name, &info.update_date, &info.install_path);
    local_data.insert_table(&info.name, table);
    local_data.write()?;
    Ok(())
}

fn font_toml_table(tag_name: &str, update_date: &str, extracted: &[PathBuf]) -> Table {
    let mut table = Table::new();
    table.insert("tag_name", value(tag_name));
    table.insert("update_date", value(update_date));

    let mut files = Array::default();
    for path in extracted {
        files.push(path.to_str().unwrap());
    }

    let files = Value::Array(files);
    table.insert("install_path", Item::Value(files));

    table
}

fn install(list: &[String]) -> Result<Vec<(String, FontStatus, FontInfo)>> {
    let catalog = Catalog::new();
    let install_list = catalog.check_args_fonts(list)?;

    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .context("Failed to build thread pool")?;

    let mp = MultiProgress::new();

    let results: Vec<_> = pool.install(|| {
        install_list
            .par_iter()
            .map(|font| (font.name.clone(), font.install(&mp)))
            .collect()
    });

    let mut font_results = Vec::new();
    let mut error_msgs = Vec::new();

    for (name, result) in results {
        match result {
            Ok(info) => {
                font_results.push((name, info.status.clone(), info.clone()));
            }
            Err(e) => error_msgs.push(format!("{}: {:#}", name, e)),
        }
    }

    // Write TOML sequentially for successful installs only
    for (name, status, info) in &font_results {
        if matches!(status, FontStatus::Success) && !info.install_path.is_empty() {
            if let Err(e) = save_font_info(info) {
                error_msgs.push(format!("{}: failed to save config: {:#}", name, e));
            }
        }
    }

    // Print any config errors that occurred during TOML writing
    for msg in &error_msgs {
        eprintln!("❌ {msg}");
    }

    Ok(font_results)
}

fn build_results_vec(
    results: Vec<(String, Result<FontInfo>)>,
) -> Vec<(String, FontStatus, FontInfo)> {
    results
        .into_iter()
        .filter_map(|(name, result)| match result {
            Ok(info) => Some((name, info.status.clone(), info)),
            Err(e) => {
                eprintln!("❌ {}: {:#}", name, e);
                None
            }
        })
        .collect()
}

fn update(list: &[String]) -> Result<Vec<(String, FontStatus, FontInfo)>> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let update_list = catalog.check_args_fonts(&list)?;

    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .context("Failed to build thread pool")?;

    let mp = MultiProgress::new();

    let results: Vec<_> = pool.install(|| {
        update_list
            .par_iter()
            .map(|font| (font.name.clone(), font.update(&mp)))
            .collect()
    });

    let font_results = build_results_vec(results);

    // Write TOML sequentially for successful updates
    for (name, status, info) in &font_results {
        if matches!(status, FontStatus::Success) && !info.install_path.is_empty() {
            if let Err(e) = save_font_info(info) {
                eprintln!("❌ {} failed to save config: {:#}", name, e);
            }
        }
    }

    Ok(font_results)
}

fn reinstall(list: &[String]) -> Result<Vec<(String, FontStatus, FontInfo)>> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let reinstall_list = catalog.check_args_fonts(&list)?;

    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .context("Failed to build thread pool")?;

    let mp = MultiProgress::new();

    let results: Vec<_> = pool.install(|| {
        reinstall_list
            .par_iter()
            .map(|font| (font.name.clone(), font.reinstall(&mp)))
            .collect()
    });

    let font_results = build_results_vec(results);

    // Write TOML sequentially for successful reinstalls
    for (name, status, info) in &font_results {
        if matches!(status, FontStatus::Success) && !info.install_path.is_empty() {
            if let Err(e) = save_font_info(info) {
                eprintln!("❌ {} failed to save config: {:#}", name, e);
            }
        }
    }

    Ok(font_results)
}

fn uninstall(list: &[String]) -> Result<Vec<(String, FontStatus)>> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let uninstall_list = catalog.check_args_fonts(&list)?;

    let local_data = Mutex::new(LocalData::new());

    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .context("Failed to build thread pool")?;

    let results: Vec<_> = pool.install(|| {
        uninstall_list
            .par_iter()
            .map(|font| {
                let data = local_data.lock().unwrap();
                (font.name.clone(), font.uninstall(data))
            })
            .collect()
    });

    let mut font_results = Vec::new();
    for (name, result) in results {
        match result {
            Ok(_) => font_results.push((name, FontStatus::Success)),
            Err(e) => font_results.push((name, FontStatus::Error(format!("{:#}", e)))),
        }
    }

    Ok(font_results)
}

fn check_list_helper(list: &[String]) -> Result<Vec<String>> {
    let mut list: Vec<String> = list.to_vec();
    if list.iter().any(|s| s.as_str() == "all") {
        if list.len() > 1 {
            eprintln!(r#"warning: when you choose "all" other options are ignored."#);
        }
        let local_data = LocalData::new();
        let config = local_data.local_data;

        list = flat_table_names(&config)?;
    }
    Ok(list)
}

fn info(list: &[String]) -> Result<()> {
    let catalog = Catalog::new();
    let to_show = catalog.check_args_fonts(list)?;
    for f in to_show {
        f.info();
    }
    Ok(())
}

/// Pretty‐print the catalog of fonts (sorted by publisher).
fn show_supported_fonts() {
    let catalog = Catalog::new();
    let fonts = catalog.fonts;
    println!("available fonts\n{:─^70}", "");
    for (index, font) in fonts.iter().enumerate() {
        let publisher = if index == 0 || font.publisher_name != fonts[index - 1].publisher_name {
            &font.publisher_name
        } else {
            ""
        };
        println!(" {:10} │ {:18} │ {}", font.name, publisher, font.repo_url);
        if index + 1 < fonts.len() && font.publisher_name != fonts[index + 1].publisher_name {
            println!(" {0:─^10}─┼{0:─^19}─┼{0:─^55}", "─");
        }
    }
}

/// Placeholder – replace with a real list of "all installed fonts".
fn list_installed_fonts() -> Result<()> {
    let local_data = LocalData::new();
    let config = local_data.local_data;

    let root = config.as_table();
    let mut list = Vec::new();

    // `root.iter()` yields (&Key, &Item) pairs.
    for (key, item) in root.iter() {
        // Keep only entries that are tables (`[foo]`).
        if matches!(item, Item::Table(_)) {
            list.push((key.to_string(), root[key]["tag_name"].as_str().unwrap()));
        }
    }

    for (font, tag) in list {
        println!("{font:10}, {:?}", tag);
    }

    Ok(())
}

/// Extract the names of every **top‐level** table from a mutable TOML document.
///
/// The function consumes the `DocumentMut` and returns it together with the
/// collected table names, so the caller can continue to edit the document.
fn flat_table_names(config: &DocumentMut) -> Result<Vec<String>> {
    // Get mutable access to the root table.
    let root = config.as_table();
    let mut names = Vec::new();

    // `root.iter()` yields (&Key, &Item) pairs.
    for (key, item) in root.iter() {
        // Keep only entries that are tables (`[foo]`).
        if matches!(item, Item::Table(_)) {
            // `key` implements `Display`, so `to_string()` gives us the key name.
            names.push(key.to_string());
        }
    }

    Ok(names)
}

fn main() {
    let cli = Cli::parse();

    let _ = run(cli.command);
}
