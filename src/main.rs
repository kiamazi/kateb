mod catalog;
mod font;
mod local_data;

use std::env;

use anyhow::{Result, anyhow};
use toml_edit::{DocumentMut, Item};

use crate::catalog::Catalog;
use crate::local_data::LocalData;

//-----------------------------
/// All commands accepted by the CLI.
#[derive(Debug)]
enum Command {
    Install(Vec<String>),
    Update(Vec<String>),
    Reinstall(Vec<String>),
    Uninstall(Vec<String>),
    List,
    Fonts,
    Info(Vec<String>),
    Version,
    SelfUpgrade,
}

impl Command {
    /// Parse the raw iterator (`env::args().skip(1)`) into a `Command`.
    fn from_iter<I>(mut it: I) -> Result<Self>
    where
        I: Iterator<Item = String>,
    {
        let cmd = it.next().ok_or(anyhow!("missing command"))?;
        let args: Vec<String> = it.collect();

        match cmd.as_str() {
            "install" => Ok(Command::Install(args)),
            "update" => Ok(Command::Update(args)),
            "reinstall" => Ok(Command::Reinstall(args)),
            "uninstall" => Ok(Command::Uninstall(args)),
            "list" => Ok(Command::List),
            "fonts" => Ok(Command::Fonts),
            "info" => Ok(Command::Info(args)),
            "version" | "-v" => Ok(Command::Version),
            "self-upgrade" => Ok(Command::SelfUpgrade),
            _ => Err(anyhow!("unknown command")),
        }
    }
}

//-----------------------------
fn run(command: Command) -> Result<(), ()> {
    match command {
        Command::Install(list) => install(&list).map_err(|e| eprintln!("❌ {:#}", e))?,
        Command::Update(list) => update(&list).map_err(|e| eprintln!("❌ {:#}", e))?,
        Command::Reinstall(list) => reinstall(&list).map_err(|e| eprintln!("❌ {:#}", e))?,
        Command::Uninstall(list) => uninstall(&list).map_err(|e| eprintln!("❌ {:#}", e))?,
        Command::Info(list) => info(&list).map_err(|e| eprintln!("❌ {:#}", e))?,
        Command::List => show_supported_fonts(),
        Command::Fonts => list_installed_fonts(),
        Command::Version => println!("Version: {}", env!("CARGO_PKG_VERSION")),
        Command::SelfUpgrade => println!("self‑upgrade…"),
    }
    Ok(())
}

fn install(list: &[String]) -> Result<()> {
    let catalog = Catalog::new();
    let install_list = catalog.check_args_fonts(list)?;

    for font in install_list {
        match font.install() {
            Ok(_) => (),
            Err(e) => eprintln!("❌ {:#}", e),
        }
    }
    Ok(())
}

fn update(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let update_list = catalog.check_args_fonts(&list)?;

    for font in update_list {
        match font.update() {
            Ok(_) => (),
            Err(e) => eprintln!("❌ {:#}", e),
        }
    }
    Ok(())
}

fn reinstall(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let reinstall_list = catalog.check_args_fonts(&list)?;

    for font in reinstall_list {
        match font.reinstall() {
            Ok(_) => (),
            Err(e) => eprintln!("❌ {:#}", e),
        }
    }
    Ok(())
}

fn uninstall(list: &[String]) -> Result<()> {
    let list: Vec<String> = check_list_helper(list)?;
    let catalog = Catalog::new();
    let update_list = catalog.check_args_fonts(&list)?;

    for font in update_list {
        match font.uninstall() {
            Ok(_) => (),
            Err(e) => eprintln!("❌ {:#}", e),
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

/// Pretty‑print the catalog of fonts (sorted by publisher).
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

/// Placeholder – replace with a real list of “all supported fonts”.
fn list_installed_fonts() {
    println!("(supported fonts list would go here)");
}

/// Extract the names of every **top‑level** table from a mutable TOML document.
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

fn usage() -> ! {
    println!(
        r#"
kateb <command> [option]

commands:
    install          install a new font
    update           update an installed font
    reinstall        reinstall an already‑installed font
    uninstall        uninstall an already‑installed font
    list             list all supported Farsi fonts
    fonts            show the fonts that are currently installed
    info             display brief information about a font’s publisher
    version | -v     display the kateb version
    self-upgrade     upgrade the kateb tool itself

options:
    -a | all         install or update every font
    <font name>      install or update the specified font

sample:
    kateb install all
"#
    );

    std::process::exit(1);
}

//-----------------------------
fn main() {
    // Parse CLI arguments
    let args = env::args().skip(1);
    let command = match Command::from_iter(args) {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("Error: {msg}");
            usage();
        }
    };

    // Execute the command; any error already printed inside `run`
    let _ = run(command);
}
