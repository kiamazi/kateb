#![allow(unused)]

mod catalog;

use std::env;
use catalog::{Font, FontError, build_catalog};

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
fn run(command: Command, fonts: &[Font]) -> Result<(), ()> {
    match command {
        Command::Install(list)   => install(fonts, &list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Update(list)    => update(fonts, &list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Reinstall(list) => reinstall(fonts, &list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::Info(list)      => info(fonts, &list).map_err(|e| eprintln!("❌ {}", e))?,
        Command::List            => list_fonts(fonts),
        Command::Fonts           => show_supported_fonts(),
        Command::Version         => println!("Version: {}", env!("CARGO_PKG_VERSION")),
        Command::SelfUpgrade     => println!("self‑upgrade…"),
    }
    Ok(())
}

/// Validate a user‑supplied list of font names and return the matching fonts.
///
/// * **Empty list** → `Err(FontError::EmptyList)`.
/// * **`"all"`** → returns **all** fonts (extra items are ignored, a warning is printed).
/// * **Invalid names** → `Err(FontError::InvalidFonts)` containing the unknown names.
/// * **Valid subset** → `Ok(Vec<&Font>)` with the matching fonts, preserving the
///   order of the original `fonts` slice.
///
/// The function never mutates its inputs.
pub fn check_fonts<'a>(
    fonts: &'a [Font],
    list: &[String],
) -> Result<Vec<&'a Font>, FontError> {
    if list.is_empty() {
        return Err(FontError::EmptyList);
    }

    // “all” handling – warning if other items are present
    if list.iter().any(|s| s.as_str() == "all") {
        if list.len() > 1 {
            eprintln!(
                r#"warning: when you choose "all" other options are ignored."#
            );
        }
        return Ok(fonts.iter().collect());
    }

    // Find unknown names
    let not_valid: Vec<String> = list
        .iter()
        .filter(|name| !fonts.iter().any(|f| f.name == **name))
        .cloned()
        .collect();

    if !not_valid.is_empty() {
        return Err(FontError::InvalidFonts(not_valid));
    }

    // All names exist → collect the matching fonts
    let res: Vec<&Font> = fonts
        .iter()
        .filter(|f| list.contains(&f.name))
        .collect();

    Ok(res)
}

/// Installs the requested fonts.
///
/// This function is a thin wrapper around [`check_fonts`] that
///
/// * validates the input list,
/// * reports any problems to the user,
/// * and, when the list is valid, calls `Font::install` for each selected font.
///
/// # Parameters
///
/// * `fonts` – A slice containing **all** fonts that are available for installation.
///   The slice is borrowed; ownership remains with the caller.
///
/// * `list` – A slice of font names supplied by the user.  The semantics are the
///   same as in [`check_fonts`]:
///
///   - If the slice contains the literal string `"all"` the function installs **every**
///     font in `fonts`.  If other items are present they are ignored, and a warning
///     is printed to `stderr`.
///   - If the slice is empty, an `EmptyList` error is returned and reported to the user.
///   - If any name does not correspond to an existing font, an `InvalidFonts` error
///     containing the offending names is returned and reported.
///   - Otherwise the function installs only the fonts whose names appear in `list`,
///     preserving the order of `fonts`.
///
/// # Returns
///
/// * `Ok(())` – All requested fonts were installed successfully.
///
/// * `Err(FontError)` – Validation failed.  The error is printed to `stderr` so the
///   user sees a helpful message, and the same error value is also returned for
///   programmatic handling by the caller.
///
/// # Example
///
/// ```rust
/// # use mycrate::{Font, install, FontError};
///
/// // Install a single, valid font
/// install(&available_fonts, &["vazir".into()]).unwrap();
///
/// // Request a non‑existent font – prints an error and returns Err
/// if let Err(e) = install(&available, &["Times".into()]) {
///     eprintln!("installation failed: {}", e);
/// }
/// ```
///
/// # Errors
///
/// See [`FontError`] for the concrete variants that can be returned:
///
/// * `EmptyList` – the user supplied no font names.
/// * `InvalidFonts` – one or more supplied names are not present in `fonts`.
///
/// The function never returns an error for the `"all"` case; it merely emits a
/// warning when additional names accompany `"all"` and proceeds with installation.
///
/// # Side effects
///
/// For each font selected by the validation step, `font.install()` is called,
/// which is expected to perform the actual installation (e.g., copying files,
/// updating a registry, etc.).  Any side effects produced by `install` are
/// therefore performed only after successful validation.
///
/// # See also
///
/// * [`check_fonts`] – the helper that performs the validation and selection logic.
fn install(fonts: &[Font], list: &[String]) -> Result<(), FontError> {
    let install_list = check_fonts(fonts, list)?;
    for font in install_list {
        font.install();
    }
    Ok(())
}

fn update(fonts: &[Font], list: &[String]) -> Result<(), FontError> {
    let to_do = check_fonts(fonts, list)?;
    for f in to_do {
        f.update();
    }
    Ok(())
}

fn reinstall(fonts: &[Font], list: &[String]) -> Result<(), FontError> {
    let to_do = check_fonts(fonts, list)?;
    for f in to_do {
        f.reinstall();
    }
    Ok(())
}

fn info(fonts: &[Font], list: &[String]) -> Result<(), FontError> {
    let to_show = check_fonts(fonts, list)?;
    for f in to_show {
        f.info();
    }
    Ok(())
}

/// Pretty‑print the catalog of fonts (sorted by publisher).
fn list_fonts(fonts: &[Font]) {
    println!("available fonts\n{:─^70}", "");
    for (i, f) in fonts.iter().enumerate() {
        let publisher = if i == 0 || f.publisher_name != fonts[i - 1].publisher_name {
            &f.publisher_name
        } else {
            ""
        };
        println!(" {:10} │ {:18} │ {}", f.name, publisher, f.repo_url);
        if i + 1 < fonts.len() && f.publisher_name != fonts[i + 1].publisher_name {
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
    let mut fonts = build_catalog();
    fonts.sort_by(|a, b| a.publisher_name.cmp(&b.publisher_name));

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
    let _ = run(command, &fonts);
}
