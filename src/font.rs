use anyhow::{Context, Result, anyhow, bail};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Read, Write, copy};
use std::path::{Path, PathBuf};
use toml_edit::{Array, Item, Table, Value, value};

use crate::local_data::LocalData;

/// Own wrapper around the GitHub releases array.
/// The array of releases returned by `GET /repos/:owner/:repo/releases`.
#[derive(Debug, Deserialize, Default)]
pub struct GithubReleases(pub Vec<Release>);

#[derive(Debug, Deserialize)]
pub struct Release {
    #[serde(rename = "tag_name")]
    pub tag_name: String,

    #[serde(rename = "updated_at")]
    pub updated_at: String, // keep as String or change to DateTime<Utc>

    /// Assets attached to the release. We only look at the first one.
    #[serde(default)]
    pub assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
pub struct Asset {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "browser_download_url")]
    pub url: String,
}

/* ---------- helper methods ---------- */

impl GithubReleases {
    /// Returns the **first** element of the releases array (the newest release,
    /// because the API returns releases sorted newest‑first by default).
    pub fn latest_release(&self) -> Option<&Release> {
        self.0.get(0)
    }
}

#[allow(unused)]
impl Release {
    /// Returns the first asset, if the release contains any.
    pub fn first_asset(&self) -> Option<&Asset> {
        self.assets.get(0)
    }

    /// Returns the first asset, if the release contains any.
    pub fn second_asset(&self) -> Option<&Asset> {
        self.assets.get(1)
    }

    /// Returns the asset, if the release contains any.
    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        if name.eq("arad") {
            self.assets.get(1)
        } else {
            self.assets.get(0)
        }
    }

    pub fn get_nth_asset(&self, index: usize) -> Option<&Asset> {
        self.assets.get(index)
    }
}

// ------------------------------------------------------

/// Information about a single font.
#[derive(Clone, Debug, PartialEq, Eq, Ord)]
pub struct Font {
    pub name: String,
    pub api: String,
    pub repo_name: String,
    pub repo_url: String,
    pub publisher_name: String,
    pub publisher_url: String,
    pub direct_download: Option<String>,
    pub extract_regex: Option<String>,
}

impl PartialOrd for Font {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.publisher_name == other.publisher_name {
            return Some(self.name.cmp(&other.name));
        }
        Some(self.publisher_name.cmp(&other.publisher_name))
    }
}

#[derive(PartialEq)]
enum FontInstall {
    Install,
    Update,
    ReInstall,
}

#[allow(unused)]
impl Font {
    pub fn install(&self) -> Result<()> {
        // return Err(anyhow!("test error"));
        self.handle_with_helper(FontInstall::Install)?;
        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        self.handle_with_helper(FontInstall::Update)?;
        Ok(())
    }

    pub fn reinstall(&self) -> Result<()> {
        self.handle_with_helper(FontInstall::ReInstall)?;
        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        let mut local_data = LocalData::new();

        if local_data.configs.contains_table(&self.name) {
            let font_table = local_data.configs[&self.name].as_table().unwrap();

            let install_path = font_table.get("install_path").unwrap().as_array().unwrap();
            let paths: Vec<String> = install_path
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                .collect();

            for path in paths {
                std::fs::remove_file(&path).ok();
            }
            local_data.configs.remove(&self.name);
            local_data.write()?;
        }
        Ok(())
    }

    pub fn info(&self) {
        println!(
            "publisher {}, {}\nrepo {}",
            self.publisher_name, self.publisher_url, self.repo_url
        );
    }

    fn handle_with_helper(&self, mode: FontInstall) -> Result<()> {
        let mut local_data = LocalData::new();
        let temp_root = tempfile::Builder::new()
            .prefix("kateb")
            .tempdir()
            .context("Unable to create temporary directory")?;
        let temp_dir = temp_root.path().to_path_buf();

        let github_api = self.fetch_api()?;
        let latest_release = github_api
            .latest_release()
            .ok_or_else(|| anyhow!("no release found for {}, try again later!", self.name))?;

        let install_cofirmed =
            if local_data.configs.contains_table(&self.name) && mode == FontInstall::ReInstall {
                true
            } else if local_data.configs.contains_table(&self.name) {
                let font_item = local_data.configs.get(&self.name).unwrap();
                let font_table = font_item.as_table().unwrap();

                match font_table.get("update_date") {
                    Some(v) => {
                        let current_version_date = parse_iso(v.as_str().unwrap());
                        let release_date = parse_iso(&latest_release.updated_at);
                        if current_version_date >= release_date {
                            println!(
                                "{:10} is already up‑to‑date ({})",
                                &self.name, latest_release.tag_name
                            );
                            return Ok(());
                        } else {
                            true
                        }
                    }
                    None => true,
                }
            } else {
                match mode {
                    FontInstall::Install => true,
                    FontInstall::Update | FontInstall::ReInstall => {
                        println!("{} is not installed", &self.name);
                        false
                    }
                }
            };

        if !install_cofirmed {
            return Ok(());
        }

        if let Some(url) = &self.direct_download {
            let file_name = url.rsplit('/').next().unwrap();
            // #TODO: replace cache_dir with target_dir
            let font_file_path = &local_data.cache_dir.join(file_name);

            download_file(&url, font_file_path)?;

            let extracted = Vec::from([font_file_path.to_owned()]);
            let table = Self::toml_table(
                &latest_release.tag_name,
                &latest_release.updated_at,
                &extracted,
            );

            local_data.insert(&self.name, Item::Table(table));
            local_data.write()?;

            return Ok(());
        }

        let asset = latest_release
            .get_asset(&self.name)
            .with_context(|| format!("no release found for {}, try again later!", &self.name))?;

        let zip_file_path = temp_dir.join(&asset.name);

        let _ = download_file(&asset.url, &zip_file_path)?;

        let pattern: &str = self.extract_regex.as_ref().unwrap().as_str();
        let extracted = unzip_file(
            &zip_file_path,
            // #TODO: target_dir
            &local_data.cache_dir,
            pattern,
        )?;

        let table = Self::toml_table(
            &latest_release.tag_name,
            &latest_release.updated_at,
            &extracted,
        );

        local_data.insert(&self.name, Item::Table(table));
        local_data.write()?;

        Ok(())
    }

    fn toml_table(tag_name: &str, update_date: &str, extracted: &Vec<PathBuf>) -> Table {
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

    fn fetch_api(&self) -> Result<GithubReleases> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("kateb/0.1")
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes
            .build()
            .context("client build failed")?;

        let response = client
            .get(&self.api)
            .send()
            .with_context(|| format!("Failed to download from {}", self.api))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default(); // read body for debugging
            bail!("Bad status {status} from {}: {}", self.api, body);
        }

        let json = response
            .json::<GithubReleases>()
            .map_err(|e| anyhow!("Failed to parse JSON from {}: {e}", self.api))?;
        Ok(json)
    }
}

// --------------------------------------------------------

fn parse_iso(s: &str) -> DateTime<Utc> {
    // `parse_from_rfc3339` understands the `…Z` suffix (UTC)
    DateTime::parse_from_rfc3339(s)
        .expect("invalid RFC‑3339 timestamp")
        .with_timezone(&Utc)
}

fn download_file(url: &str, destination: &PathBuf) -> Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300)) // 5 minutes
        .build()
        .context("Failed to build reqwest client")?;

    let mut response = client
        .get(url)
        .send()
        .with_context(|| format!("Failed to download from {}", url))?;

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("=> "),
    );


    let mut file = File::create(destination)
        .with_context(|| format!("Failed to create file {:?}", destination))?;

    let mut buffer = [0u8; 8192]; // 8 KB per iteration
    loop {
        let n = response
            .read(&mut buffer)
            .expect("failed while reading response body");
        if n == 0 {
            break; // EOF
        }
        file.write_all(&buffer[..n])
            .with_context(|| format!("failed to write to file {:?}", destination))?;

        pb.inc(n as u64);
    }

    // ------------------------------------------------------------------

    pb.finish_and_clear();
    let size_display = if total_size == 0 {
        "unknown".to_string()
    } else {
        total_size.to_string()
    };

    println!(
        "✅ Downloaded {} ({} bytes)",
        destination.file_name().unwrap().display(),
        size_display
    );

    Ok(())
}

fn unzip_file(file: &PathBuf, target_dir: &PathBuf, pattern: &str) -> Result<Vec<PathBuf>> {
    use regex::Regex;
    use zip::read::ZipArchive;

    let zip_file = File::open(file)?;
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;
    let re = Regex::new(pattern).expect("regex is constant and valid");

    let mut extracted = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        // `entry.name()` returns the path stored in the zip (always uses `/` as separator)
        let entry_name = entry.name();

        // Skip directories
        if entry.is_dir() {
            continue;
        }

        // Does the entry match the wanted pattern?
        let caps = match re.captures(entry_name) {
            Some(c) => c,
            None => continue, // not a file we care about
        };

        // `caps[1]` is the *file name* part (no directories)
        let file_name = Path::new(&caps[1]);

        // Build the full destination path (target_dir / file_name)
        let dest_path = target_dir.join(file_name);

        // -------------------------------------------------
        //  Stream the entry's contents into the destination file
        // -------------------------------------------------
        let mut out = File::create(&dest_path)
            .with_context(|| format!("cannot create `{}`", dest_path.display()))?;
        let _ = copy(&mut entry, &mut out)
            .with_context(|| format!("failed to write `{}`", dest_path.display()))?;

        extracted.push(dest_path);
    }

    // println!("{:?}", extracted);
    Ok(extracted)
}
