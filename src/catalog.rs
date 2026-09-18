use anyhow::{Result, anyhow, bail};
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::font::Font;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Catalog {
    pub fonts: Vec<Font>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog {
            fonts: build_catalog(),
        }
    }

    pub fn font_list(&self) -> Vec<&String> {
        let mut list = Vec::new();
        for font in &self.fonts {
            list.push(&font.name);
        }
        list
    }

    /// Validate a user-supplied list of font names and return the matching fonts.
    pub fn check_args_fonts<'a>(&'a self, list: &[String]) -> Result<Vec<&'a Font>> {
        if list.is_empty() {
            bail!("the font list is empty – please specify at least one font name");
        }

        let fonts = Self::font_list(self);

        // "all" handling – warning if other items are present
        if list.iter().any(|s| s.as_str() == "all") {
            if list.len() > 1 {
                eprintln!(r#"warning: when you choose "all" other options are ignored."#);
            }
            return Ok(self.fonts.iter().collect());
        }

        // Find unknown names
        let not_valid: Vec<String> = list
            .iter()
            .filter(|name| !fonts.contains(name))
            .cloned()
            .collect();

        if !not_valid.is_empty() {
            bail!("these fonts are not valid: {:?}", not_valid);
        }

        // All names exist → collect the matching fonts
        let res: Vec<&Font> = self
            .fonts
            .iter()
            .filter(|f| list.contains(&f.name))
            .collect();

        Ok(res)
    }
}

#[derive(Deserialize)]
struct CatalogToml {
    fonts: Vec<FontEntry>,
}

#[derive(Deserialize)]
struct FontEntry {
    name: String,
    api: String,
    repo_name: String,
    repo_url: String,
    publisher_name: String,
    publisher_url: String,
    #[serde(default)]
    direct_download: Option<String>,
    #[serde(default)]
    extract_regex: Option<String>,
    #[serde(default = "default_asset_number")]
    asset_number: usize,
}

fn default_asset_number() -> usize {
    0
}

fn build_catalog() -> Vec<Font> {
    // ====== Development-only code – kept for testing ======
    // let catalog_path = std::path::Path::new("catalog.toml");
    // let toml_str = std::fs::read_to_string(catalog_path).expect("Failed to read catalog.toml");
    // ======================================================

    let toml_str = fetch_catalog(
        "https://raw.githubusercontent.com/kiamazi/kateb/refs/heads/main/catalog.toml",
    )
    .unwrap_or_else(|err| {
        eprintln!("Error fetching TOML: {}", err);
        std::process::exit(1)
    });

    let catalog: CatalogToml = toml::from_str(&toml_str).expect("Failed to parse catalog.toml");

    let mut fonts: Vec<Font> = catalog
        .fonts
        .into_iter()
        .map(|entry| Font {
            name: entry.name,
            api: entry.api,
            repo_name: entry.repo_name,
            repo_url: entry.repo_url,
            publisher_name: entry.publisher_name,
            publisher_url: entry.publisher_url,
            direct_download: entry.direct_download.filter(|s| !s.is_empty()),
            extract_regex: entry.extract_regex.filter(|s| !s.is_empty()),
            asset_number: entry.asset_number,
        })
        .collect();

    fonts.sort();
    fonts
}

pub fn fetch_catalog(url: &str) -> Result<String> {
    let client = Client::new();

    let response = client
        .get(url)
        .send()
        .map_err(|e| anyhow!("request error: {}", e))?;

    if !response.status().is_success() {
        bail!("failed to fetch '{}': HTTP {}", url, response.status());
    }

    let body = response
        .text()
        .map_err(|e| anyhow!("failed to read response body: {}", e))?;

    Ok(body)
}
