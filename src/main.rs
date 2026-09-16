mod catalog;
mod font;
mod local_data;

use anyhow::Result;
use clap::{Parser, Subcommand};
use toml_edit::{DocumentMut, Item};

use crate::catalog::Catalog;
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
        Commands::Install { fonts } => install(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::Update { fonts } => update(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::Reinstall { fonts } => reinstall(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::Uninstall { fonts } => uninstall(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::Info { fonts } => info(&fonts).map_err(|e| eprintln!("❌ {:#}", e))?,
        Commands::List => show_supported_fonts(),
        Commands::Fonts => list_installed_fonts(),
        Commands::Version => println!("Version: {}", env!("CARGO_PKG_VERSION")),
        Commands::SelfUpgrade => println!("self‐upgrade..."),
    }
    Ok(())
}

fn install(list: &[String]) -> Result<()> {
    let catalog = Catalog::new();
    let install_list = catalog.check_args_fonts(list)?;

    for font in install_list {
        if let Err(msg) = font.install() {
            eprintln!("❌ {:#}", msg);
        }
    }
    Ok(())
}

fn update(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let update_list = catalog.check_args_fonts(&list)?;

    for font in update_list {
        if let Err(msg) = font.update() {
            eprintln!("❌ {:#}", msg);
        }
    }
    Ok(())
}

fn reinstall(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let reinstall_list = catalog.check_args_fonts(&list)?;

    for font in reinstall_list {
        if let Err(msg) = font.reinstall() {
            eprintln!("❌ {:#}", msg);
        }
    }
    Ok(())
}

fn uninstall(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let uninstall_list = catalog.check_args_fonts(&list)?;

    for font in uninstall_list {
        if let Err(msg) = font.uninstall() {
            eprintln!("❌ {:#}", msg);
        }
    }
    Ok(())
}

fn check_list_helper(list: &[String]) -> Result<Vec<String>> {
    let mut list: Vec<String> = list.to_vec();
    if list.iter().any(|s| s.as_str() == "all") {
        if list.len() > 1 {
            eprintln!(r#"warning: when you choose "all" other options are ignored."#);
        }
        let local_data = LocalData::new();
        let config = local_data.configs;

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

/// Placeholder – replace with a real list of "all supported fonts".
fn list_installed_fonts() {
    println!("(supported fonts list would go here)");
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
