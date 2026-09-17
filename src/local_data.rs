use anyhow::{Context, Result};
use dirs;
use nix::unistd::Uid;
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml_edit::{DocumentMut, Item, Table};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct LocalData {
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub toml_file: PathBuf,
    pub cache_dir: PathBuf,
    pub font_dir: PathBuf,
    pub local_dat: DocumentMut,
}

impl LocalData {
    pub fn new() -> Self {
        match Self::prepare() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }
    }

    fn prepare() -> Result<Self> {
        // ----- executable name -------------------------------------------------
        let exec_name = "kateb";

        // ----- HOME ------------------------------------------------------------
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to determine home directory"))?;

        // ----- CONFIG DIRECTORY ------------------------------------------------
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to locate XDG config dir"))?
            .join(exec_name);

        // ----- Data Directory ---------------------------------------------------
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to locate XDG data dir"))?
            .join(exec_name);

        // ----- CACHE (fonts) ---------------------------------------------------
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Unable to locate XDG cache dir"))?
            .join(exec_name);

        // ----- TARGET (install) DIRECTORY --------------------------------------
        // Platform‑specific defaults (macOS vs other Unix)
        let (root_font_dir, user_font_dir) = if cfg!(target_os = "macos") {
            (
                Path::new("/Library/Fonts").to_path_buf(),
                dirs::font_dir().unwrap_or(home_dir.join("Library/Fonts")),
            )
        } else {
            (
                Path::new("/usr/share/fonts/truetype/farsifreefont").to_path_buf(),
                dirs::font_dir().unwrap_or(
                    home_dir
                        .join(".local")
                        .join("share")
                        .join("fonts")
                        .join("farsifreefont"),
                ),
            )
        };

        // If we run as root (uid 0) install system‑wide, otherwise user‑wide.
        let target_font_dir = if Uid::effective().is_root() {
            root_font_dir
        } else {
            user_font_dir
        };

        // ----- TOML DATABASE FILE ----------------------------------------------
        let toml_file = config_dir.join("config.toml");

        // Ensure the config and target directories exist.
        ensure_dir(&config_dir)?;
        ensure_dir(&data_dir)?;
        ensure_dir(&cache_dir)?;
        ensure_dir(&target_font_dir)?;

        // ----- LOAD OR INITIALISE THE TOML DATABASE ----------------------------
        let local_dat: DocumentMut = if !toml_file.is_file() {
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
            home_dir,
            config_dir,
            cache_dir,
            font_dir: target_font_dir,
            data_dir,
            toml_file,
            local_dat,
        })
    }

    /// -----------------------------------------------------------------------
    /// Write a new TOML structure to the database file
    /// -----------------------------------------------------------------------
    pub fn write_data<T: ToString>(&self, local_data: &T) -> Result<()> {
        let toml = local_data.to_string();
        fs::write(&self.toml_file, toml)
            .with_context(|| format!("Unable to write {}", self.toml_file.display()))?;
        Ok(())
    }

    fn reset_toml_file(path: &Path) -> Result<DocumentMut> {
        let config: DocumentMut = DocumentMut::new(); // mut config

        let toml_string = config.to_string();
        fs::write(path, &toml_string).context(format!("Failed to write {}", path.display()))?;
        Ok(config)
    }

    pub fn insert_table(&mut self, key: &str, table: Table) {
        let table = Item::Table(table);
        self.local_dat.insert(key, table);
    }

    #[allow(unused)]
    pub fn insert_itam(&mut self, key: &str, item: Item) {
        self.local_dat.insert(key, item);
    }

    pub fn write(&self) -> Result<()> {
        self.write_data(&self.local_dat)?;
        Ok(())
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
