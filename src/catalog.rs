use std::{fmt, error::Error};

/// Information about a single font.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Font {
    pub name: String,
    pub api: String,
    pub repo_name: String,
    pub repo_url: String,
    pub publisher_name: String,
    pub publisher_url: String,
}

impl Font {
    pub fn install(&self)   { println!("install {}", self.name); }
    pub fn update(&self)    { println!("update {}", self.name); }
    pub fn reinstall(&self){ println!("reinstall {}", self.name); }
    pub fn info(&self) {
        println!(
            "publisher {}, {}\nrepo {}",
            self.publisher_name, self.publisher_url, self.repo_url
        );
    }
}

/// Errors that can arise while checking fonts.
#[derive(Debug)]
pub enum FontError {
    /// The user asked for a specific list of fonts, but some of them don’t exist.
    InvalidFonts(Vec<String>),
    /// The caller supplied an empty `list`. Nothing can be selected.
    EmptyList,
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontError::InvalidFonts(bad) => {
                write!(f, "these fonts are not valid: {:?}", bad)
            }
            FontError::EmptyList => {
                write!(f, "the font list is empty – please specify at least one font name")
            }
        }
    }
}
impl Error for FontError {}

pub fn build_catalog() -> Vec<Font> {
    vec![
        Font {
			name: "vazir".to_string(),
			api: "https://api.github.com/repos/rastikerdar/vazirmatn/tags".to_string(),
			repo_name: "vazirmatn".to_string(),
			repo_url: "https://github.com/rastikerdar/vazirmatn/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "samim".to_string(),
			api: "https://api.github.com/repos/rastikerdar/samim-font/tags".to_string(),
			repo_name: "samim-font".to_string(),
			repo_url: "https://github.com/rastikerdar/samin-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "tanha".to_string(),
			api: "https://api.github.com/repos/rastikerdar/tanha-font/tags".to_string(),
			repo_name: "tanha-font".to_string(),
			repo_url: "https://github.com/rastikerdar/tanha-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "shabnam".to_string(),
			api: "https://api.github.com/repos/rastikerdar/shabnam-font/tags".to_string(),
			repo_name: "shabnam-font".to_string(),
			repo_url: "https://github.com/rastikerdar/shabnam-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "gandom".to_string(),
			api: "https://api.github.com/repos/rastikerdar/gandom-font/tags".to_string(),
			repo_name: "gandom-font".to_string(),
			repo_url: "https://github.com/rastikerdar/gandom-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "parastoo".to_string(),
			api: "https://api.github.com/repos/rastikerdar/parastoo-font/tags".to_string(),
			repo_name: "parastoo-font".to_string(),
			repo_url: "https://github.com/rastikerdar/parastoo-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "sahel".to_string(),
			api: "https://api.github.com/repos/rastikerdar/sahel-font/tags".to_string(),
			repo_name: "sahel-font".to_string(),
			repo_url: "https://github.com/rastikerdar/sahel-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "vazircode".to_string(),
			api: "https://api.github.com/repos/rastikerdar/vazir-code-font/tags".to_string(),
			repo_name: "vazir-code-font".to_string(),
			repo_url: "https://github.com/rastikerdar/vazir-code-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "ziracode".to_string(),
			api: "https://api.github.com/repos/kiamazi/zira-code-font/tags".to_string(),
			repo_name: "zira-code-font".to_string(),
			repo_url: "https://github.com/kiamazi/zira-code-font/".to_string(),
			publisher_name: "Kiavash Mazi".to_string(),
			publisher_url: "https://github.com/kiamazi".to_string(),
		},
		Font {
			name: "nahid".to_string(),
			api: "https://api.github.com/repos/rastikerdar/nahid-font/tags".to_string(),
			repo_name: "nahid-font".to_string(),
			repo_url: "https://github.com/rastikerdar/nahid-font/".to_string(),
			publisher_name: "Saber Rastikerdar".to_string(),
			publisher_url: "https://github.com/rastikerdar".to_string(),
		},
		Font {
			name: "mikhak".to_string(),
			api: "https://api.github.com/repos/aminabedi68/Mikhak/tags".to_string(),
			repo_name: "Mikhak".to_string(),
			repo_url: "https://github.com/aminabedi68/Mikhak/".to_string(),
			publisher_name: "Amin Abedi".to_string(),
			publisher_url: "https://github.com/aminabedi68".to_string(),
		},
		Font {
			name: "estedad".to_string(),
			api: "https://api.github.com/repos/aminabedi68/Estedad/tags".to_string(),
			repo_name: "Estedad".to_string(),
			repo_url: "https://github.com/aminabedi68/Estedad/".to_string(),
			publisher_name: "Amin Abedi".to_string(),
			publisher_url: "https://github.com/aminabedi68".to_string(),
		},
		Font {
			name: "ganjnameh".to_string(),
			api: "https://api.github.com/repos/font-store/GanjnamehFont/tags".to_string(),
			repo_name: "GanjnamehFont".to_string(),
			repo_url: "https://github.com/font-store/GanjnamehFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string()
		},
		Font {
			name: "behdad".to_string(),
			api: "https://api.github.com/repos/font-store/BehdadFont/tags".to_string(),
			repo_name: "BehdadFont".to_string(),
			repo_url: "https://github.com/font-store/BehdadFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "nika".to_string(),
			api: "https://api.github.com/repos/font-store/NikaFont/tags".to_string(),
			repo_name: "NikaFont".to_string(),
			repo_url: "https://github.com/font-store/NikaFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "farbod".to_string(),
			api: "https://api.github.com/repos/font-store/FarbodFont/tags".to_string(),
			repo_name: "FarbodFont".to_string(),
			repo_url: "https://github.com/font-store/FarbodFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "shahab".to_string(),
			api: "https://api.github.com/repos/font-store/ShahabFont/tags".to_string(),
			repo_name: "ShahabFont".to_string(),
			repo_url: "https://github.com/font-store/ShahabFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "noon".to_string(),
			api: "https://api.github.com/repos/font-store/NoonFont/tags".to_string(),
			repo_name: "NoonFont".to_string(),
			repo_url: "https://github.com/font-store/NoonFont/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "pfont".to_string(),
			api: "https://api.github.com/repos/pfont/pfont/tags".to_string(),
			repo_name: "pfont".to_string(),
			repo_url: "https://github.com/pfont/pfont/".to_string(),
			publisher_name: "Persian Free Font".to_string(),
			publisher_url: "https://github.com/pfont".to_string(),
		},
		Font {
			name: "lalezar".to_string(),
			api: "https://api.github.com/repos/BornaIz/Lalezar/tags".to_string(),
			repo_name: "Lalezar".to_string(),
			repo_url: "https://github.com/BornaIz/Lalezar/".to_string(),
			publisher_name: "Borna Izadpanah".to_string(),
			publisher_url: "https://github.com/BornaIz".to_string(),
		},
		Font {
			name: "nastaliq".to_string(),
			api: "https://api.github.com/repos/font-store/font-IranNastaliq/tags".to_string(),
			repo_name: "font-IranNastaliq".to_string(),
			repo_url: "https://github.com/font-store/font-IranNastaliq/".to_string(),
			publisher_name: "Saleh Souzanchi".to_string(),
			publisher_url: "https://github.com/font-store".to_string(),
		},
		Font {
			name: "arad".to_string(),
			api: "https://api.github.com/repos/MDarvishi5124/Arad/releases".to_string(),
			repo_name: "Arad".to_string(),
			repo_url: "https://github.com/MDarvishi5124/Arad/".to_string(),
			publisher_name: "Mohammad Darvishi".to_string(),
			publisher_url: "https://github.com/MDarvishi5124".to_string(),
		},
    ]
}
