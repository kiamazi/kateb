#![allow(unused)]

mod catalog;
mod local_data;

use std::env;
use catalog::{Font, FontError, Catalog};
use serde_json::{Map, Value, json};
use toml_edit::{DocumentMut, Item, Table, value};

use crate::local_data::LocalData;

//-----------------------------
/// All commands accepted by the CLI.
#[derive(Debug)]
enum Command {
    Install(Vec<String>),
    Update(Vec<String>),
    Reinstall(Vec<String>),
    List,
    Fonts,
    Info(Vec<String>),
    Version,
    SelfUpgrade,
}

impl Command {
    /// Parse the raw iterator (`env::args().skip(1)`) into a `Command`.
    fn from_iter<I>(mut it: I) -> Result<Self, &'static str>
    where
        I: Iterator<Item = String>,
    {
        let cmd = it.next().ok_or("missing command")?;
        let args: Vec<String> = it.collect();

        match cmd.as_str() {
            "install"   => Ok(Command::Install(args)),
            "update"    => Ok(Command::Update(args)),
            "reinstall" => Ok(Command::Reinstall(args)),
            "list"      => Ok(Command::List),
            "fonts"     => Ok(Command::Fonts),
            "info"      => Ok(Command::Info(args)),
            "version" | "-v" => Ok(Command::Version),
            "self-upgrade" => Ok(Command::SelfUpgrade),
            _ => Err("unknown command"),
        }
    }
}

//-----------------------------
fn run(command: Command) -> Result<(), ()> {
    match command {
        Command::Install(list)   => install(&list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Update(list)    => update(&list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Reinstall(list) => reinstall(&list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Info(list)      => info(&list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::List            => list_fonts(),
        Command::Fonts           => show_supported_fonts(),
        Command::Version         => println!("Version: {}", env!("CARGO_PKG_VERSION")),
        Command::SelfUpgrade     => println!("self‑upgrade…"),
    }
    Ok(())
}

fn install(list: &[String]) -> Result<(), FontError> {
    let catalog = Catalog::new();
    let install_list = catalog.check_fonts(list)?;

    let local_data = LocalData::new();
    let config = local_data.configs;

    for font in install_list {
        let version_check = if !config.contains_table(&font.name) {
            true
        } else {
            false
        };
        if version_check {
            font.install();
        } else {
            println!("{} already installed", font.name);
        }
    }
    Ok(())
}

fn update(list: &[String]) -> Result<(), FontError> {
    let catalog = Catalog::new();
    let to_do = catalog.check_fonts(list)?;
    for f in to_do {
        f.update();
    }
    Ok(())
}

fn reinstall(list: &[String]) -> Result<(), FontError> {
    let catalog = Catalog::new();
    let to_do = catalog.check_fonts(list)?;
    for f in to_do {
        f.reinstall();
    }
    Ok(())
}

fn info(list: &[String]) -> Result<(), FontError> {
    let catalog = Catalog::new();
    let to_show = catalog.check_fonts(list)?;
    for f in to_show {
        f.info();
    }
    Ok(())
}

/// Pretty‑print the catalog of fonts (sorted by publisher).
fn list_fonts() {
    let catalog = Catalog::new();
    let mut fonts = catalog.fonts;
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
fn show_supported_fonts() {
    println!("(supported fonts list would go here)");
}

fn usage() -> ! {
    println!(r#"
kateb <command> [option]

commands:
    install          install a new font
    update           update an installed font
    reinstall        reinstall an already‑installed font
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
"#);

    std::process::exit(1);
}


//-----------------------------
fn main() {
    // let mut data = LocalData::new();
    // println!("{:#?}", data);
    // let mut table = Table::new();
    // table.insert("tag_name", value("v1.2.3"));
    // data.insert("shahram", Item::Table(table));
    // data.write();

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
