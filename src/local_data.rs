use anyhow::{Context, Result};
use dirs::{config_dir, home_dir};
use nix::unistd::Uid;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tempfile::TempDir;
use toml_edit::{Array, DocumentMut, Item, Table, TableLike, value};

use crate::catalog::Catalog;


/// ---------------------------------------------------------------------------
/// The main data structure – equivalent to the hash reference returned by the
/// Perl `_prepare` subroutine.
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct LocalData {
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub toml_file: PathBuf,
    pub cache_dir: PathBuf,
    pub target_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub configs: DocumentMut,
}

impl LocalData {
    /// Public constructor – mirrors Perl's `new`.
    pub fn new() -> Self {
        match Self::prepare() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    /// -----------------------------------------------------------------------
    /// Core preparation logic (a direct translation of the Perl `_prepare`).
    /// -----------------------------------------------------------------------
    fn prepare() -> Result<Self> {
        // ----- executable name -------------------------------------------------
        let exec_name = "kateb";

        // ----- HOME ------------------------------------------------------------
        let home = home_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to determine home directory"))?;

        // ----- CONFIG DIRECTORY ------------------------------------------------
        // `dirs::config_dir()` returns XDG_CONFIG_HOME if set,
        // otherwise $HOME/.config.
        let config = config_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to locate XDG config dir"))?
            .join(exec_name);

        // ----- CACHE (fonts) ---------------------------------------------------
        let cache = config.join("fonts");

        // ----- TARGET (install) DIRECTORY --------------------------------------
        // Platform‑specific defaults (macOS vs other Unix)
        let root_font_dir = if cfg!(target_os = "macos") {
            Path::new("/Library/Fonts").to_path_buf()
        } else {
            Path::new("/usr/share/fonts/truetype/farsifreefont").to_path_buf()
        };
        let user_font_dir = if cfg!(target_os = "macos") {
            dirs::font_dir().unwrap_or(
                home.join("Library/Fonts")
            )
        } else {
            dirs::font_dir().unwrap_or(
                home.join(".local")
                    .join("share")
                    .join("fonts")
                    .join("farsifreefont")
            )
        };
        // If we run as root (uid 0) install system‑wide, otherwise user‑wide.
        let target = if Uid::effective().is_root() {
            root_font_dir
        } else {
            user_font_dir
        };

        // ----- TEMPORARY DOWNLOAD DIRECTORY ------------------------------------
        // `tempfile::TempDir` creates a unique temporary folder that is removed
        // when the `TempDir` value is dropped. with a sub‑folder called
        // “kateb” inside it, exactly as the Perl code does.
        let temp_root = TempDir::new()
            .context("Unable to create temporary directory")?;
        let temp_dir = temp_root.path().join(exec_name);
        fs::create_dir_all(&temp_dir)
            .with_context(|| format!("Failed to create {}", temp_dir.display()))?;

        // ----- JSON DATABASE FILE ----------------------------------------------
        // let json_file = config.join(format!("{exec_name}.json"));
        let toml_file = config.join("config.toml");

        // Ensure the config and target directories exist.
        ensure_dir(&config)?;
        ensure_dir(&target)?;

        // ----- LOAD OR INITIALISE THE TOML DATABASE ----------------------------
        let configs: DocumentMut = if !toml_file.is_file() {
            // No file → create an empty one.
            let toml = Self::reset_toml_file(&toml_file)?;
            toml
        } else {
            // File exists → try to parse it.
            let raw = fs::read_to_string(&toml_file)
                .with_context(|| format!("Failed to read {}", toml_file.display()))?;
            match raw.parse::<DocumentMut>() {
                Ok(toml) => toml,
                Err(_) => {
                    // Corrupt JSON → reset to a clean file.
                    let toml = Self::reset_toml_file(&toml_file)?;
                    toml
                }
            }
        };

        Ok(LocalData {
            home_dir: home,
            config_dir: config,
            cache_dir: cache,
            target_dir: target,
            temp_dir,
            toml_file,
            configs,
        })
    }

    /// -----------------------------------------------------------------------
    /// Write a new TOML structure to the database file – mirrors Perl `write_data`.
    /// -----------------------------------------------------------------------
    pub fn write_data<T: ToString>(&self, configs: &T) -> Result<()> {
        let toml = configs.to_string();
        fs::write(&self.toml_file, toml)
            .with_context(|| format!("Unable to write {}", self.toml_file.display()))?;
        Ok(())
    }

    fn reset_toml_file(path: &Path) -> Result<DocumentMut> {
        let catalog = Catalog::new();
        let fonts = catalog.fonts;

        let config: DocumentMut = DocumentMut::new(); // mut config

        // for font in fonts {
        //     let mut table = Table::new();
        //     table["tag_name"] = value("");
        //     table["update_date"] = value("");
        //     table["install_path"] = Item::Value(Array::new().into());
        //     config.insert(&font.name, Item::Table(table));
        // }

        let toml_string = config.to_string();
        fs::write(path, &toml_string)
            .context(format!("Failed to write {}", path.display()))?;
        Ok(config)
    }

    pub fn insert(&mut self, key: &str, item: Item) {
        self.configs.insert(key, item);
    }

    pub fn write(&self) {
        self.write_data(&self.configs);
    }
}

/// ---------------------------------------------------------------------------
/// Small utility to ensure a directory exists (creates it recursively).
/// ---------------------------------------------------------------------------
fn ensure_dir(p: &Path) -> Result<()> {
    if !p.is_dir() {
        fs::create_dir_all(p)
            .with_context(|| format!("Unable to create directory {}", p.display()))?;
    }
    Ok(())
}
