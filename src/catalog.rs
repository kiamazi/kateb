use anyhow::{Result, bail};

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

    pub fn font_list<'a>(&'a self) -> Vec<&'a String> {
        let mut fonts = self.fonts.clone();
        let mut list = Vec::new();
        for font in &self.fonts {
            list.push(&font.name);
        }
        list
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
    pub fn check_args_fonts<'a>(&'a self, list: &[String]) -> Result<Vec<&'a Font>> {
        if list.is_empty() {
            bail!("the font list is empty – please specify at least one font name");
        }

        let fonts = Self::font_list(self);

        // “all” handling – warning if other items are present
        if list.iter().any(|s| s.as_str() == "all") {
            if list.len() > 1 {
                eprintln!(r#"warning: when you choose "all" other options are ignored."#);
            }
            return Ok(self.fonts.iter().collect());
        }

        // Find unknown names
        let not_valid: Vec<String> = list
            .iter()
            .filter(|name| !fonts.contains(name)) //iter().any(|font| font == name))
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

fn build_catalog() -> Vec<Font> {
    let mut fonts = vec![
        Font {
			name: "vazir".to_string(),
			api: "https://api.github.com/repos/rastikerdar/vazirmatn/releases".to_string(),
			repo_name: "vazirmatn".to_string(),
			repo_url: "https://github.com/rastikerdar/vazirmatn/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^fonts/ttf/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "samim".to_string(),
			api: "https://api.github.com/repos/rastikerdar/samim-font/releases".to_string(),
			repo_name: "samim-font".to_string(),
			repo_url: "https://github.com/rastikerdar/samin-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "tanha".to_string(),
			api: "https://api.github.com/repos/rastikerdar/tanha-font/releases".to_string(),
			repo_name: "tanha-font".to_string(),
			repo_url: "https://github.com/rastikerdar/tanha-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "shabnam".to_string(),
			api: "https://api.github.com/repos/rastikerdar/shabnam-font/releases".to_string(),
			repo_name: "shabnam-font".to_string(),
			repo_url: "https://github.com/rastikerdar/shabnam-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "gandom".to_string(),
			api: "https://api.github.com/repos/rastikerdar/gandom-font/releases".to_string(),
			repo_name: "gandom-font".to_string(),
			repo_url: "https://github.com/rastikerdar/gandom-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "parastoo".to_string(),
			api: "https://api.github.com/repos/rastikerdar/parastoo-font/releases".to_string(),
			repo_name: "parastoo-font".to_string(),
			repo_url: "https://github.com/rastikerdar/parastoo-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^web/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "sahel".to_string(),
			api: "https://api.github.com/repos/rastikerdar/sahel-font/releases".to_string(),
			repo_name: "sahel-font".to_string(),
			repo_url: "https://github.com/rastikerdar/sahel-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "vazircode".to_string(),
			api: "https://api.github.com/repos/rastikerdar/vazir-code-font/releases".to_string(),
			repo_name: "vazir-code-font".to_string(),
			repo_url: "https://github.com/rastikerdar/vazir-code-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "ziracode".to_string(),
			api: "https://api.github.com/repos/kiamazi/zira-code-font/releases".to_string(),
			repo_name: "zira-code-font".to_string(),
			repo_url: "https://github.com/kiamazi/zira-code-font/".to_string(),
			publisher_name: "Kiavash Mazi".to_string(),
			publisher_url: "https://github.com/kiamazi".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "nahid".to_string(),
			api: "https://api.github.com/repos/rastikerdar/nahid-font/releases".to_string(),
			repo_name: "nahid-font".to_string(),
			repo_url: "https://github.com/rastikerdar/nahid-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "mikhak".to_string(),
			api: "https://api.github.com/repos/aminabedi68/Mikhak/releases".to_string(),
			repo_name: "Mikhak".to_string(),
			repo_url: "https://github.com/aminabedi68/Mikhak/".to_string(),
			publisher_name: "Amin Abedi".to_string(),
			publisher_url: "https://github.com/aminabedi68".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^ttf/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "estedad".to_string(),
			api: "https://api.github.com/repos/aminabedi68/Estedad/releases".to_string(),
			repo_name: "Estedad".to_string(),
			repo_url: "https://github.com/aminabedi68/Estedad/".to_string(),
			publisher_name: "Amin Abedi".to_string(),
			publisher_url: "https://github.com/aminabedi68".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^Statics/ttf/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "ganjnameh".to_string(),
			api: "https://api.github.com/repos/font-store/GanjnamehFont/releases".to_string(),
			repo_name: "GanjnamehFont".to_string(),
			repo_url: "https://github.com/font-store/GanjnamehFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^fonts/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "behdad".to_string(),
			api: "https://api.github.com/repos/font-store/BehdadFont/releases".to_string(),
			repo_name: "BehdadFont".to_string(),
			repo_url: "https://github.com/font-store/BehdadFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "nika".to_string(),
			api: "https://api.github.com/repos/font-store/NikaFont/releases".to_string(),
			repo_name: "NikaFont".to_string(),
			repo_url: "https://github.com/font-store/NikaFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^fonts/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "farbod".to_string(),
			api: "https://api.github.com/repos/font-store/FarbodFont/releases".to_string(),
			repo_name: "FarbodFont".to_string(),
			repo_url: "https://github.com/font-store/FarbodFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "shahab".to_string(),
			api: "https://api.github.com/repos/font-store/ShahabFont/releases".to_string(),
			repo_name: "ShahabFont".to_string(),
			repo_url: "https://github.com/font-store/ShahabFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^fonts/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "noon".to_string(),
			api: "https://api.github.com/repos/font-store/NoonFont/releases".to_string(),
			repo_name: "NoonFont".to_string(),
			repo_url: "https://github.com/font-store/NoonFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "pfont".to_string(),
			api: "https://api.github.com/repos/pfont/pfont/releases".to_string(),
			repo_name: "pfont".to_string(),
			repo_url: "https://github.com/pfont/pfont/".to_string(),
			publisher_name: "Persian Free Font".to_string(),
			publisher_url: "https://github.com/pfont".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^pfont/ttf/Hinted/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "lalezar".to_string(),
			api: "https://api.github.com/repos/BornaIz/Lalezar/releases".to_string(),
			repo_name: "Lalezar".to_string(),
			repo_url: "https://github.com/BornaIz/Lalezar/".to_string(),
			publisher_name: "Borna Izadpanah".to_string(),
			publisher_url: "https://github.com/BornaIz".to_string(),
			direct_download: Some("https://raw.githubusercontent.com/BornaIz/Lalezar/master/fonts/Lalezar-Regular.ttf".to_string()),
			extract_regex: None,
		},
		Font {
			name: "nastaliq".to_string(),
			api: "https://api.github.com/repos/font-store/font-IranNastaliq/releases".to_string(),
			repo_name: "font-IranNastaliq".to_string(),
			repo_url: "https://github.com/font-store/font-IranNastaliq/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
			direct_download: Some("https://github.com/font-store/font-IranNastaliq/raw/master/WebFonts/IranNastaliq-Web.ttf".to_string()),
			extract_regex: None,
		},
		Font {
			name: "arad".to_string(),
			api: "https://api.github.com/repos/MohamadDarvishi/Arad/releases".to_string(),
			repo_name: "Arad".to_string(),
			repo_url: "https://github.com/MohamadDarvishi/Arad".to_string(),
			publisher_name: "Mohammad Darvishi".to_string(),
			publisher_url: "https://github.com/MohamadDarvishi".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^MainFonts.*/Static_TTF/([^/]+\.ttf)$"#.to_string()),
		},
		Font {
			name: "ario".to_string(),
			api: "https://api.github.com/repos/MohamadDarvishi/Ario/releases".to_string(),
			repo_name: "Ario".to_string(),
			repo_url: "https://github.com/MohamadDarvishi/Ario".to_string(),
			publisher_name: "Mohammad Darvishi".to_string(),
			publisher_url: "https://github.com/MohamadDarvishi".to_string(),
			direct_download: None,
			extract_regex: Some(r#"^Main_Fonts.*/([^/]+\.ttf)$"#.to_string()),
		},
    ];
    // fonts.sort_by(|a, b| a.publisher_name.cmp(&b.publisher_name));
    fonts.sort();
    fonts
}
