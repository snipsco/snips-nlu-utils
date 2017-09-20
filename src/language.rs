use std::str::FromStr;

const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
const SPACE: &str = " ";

macro_rules! language_enum {
    ([$($language:ident),*]) => {
        /// Enumerates all language supported
        #[derive(Copy,Clone,Debug)]
        pub enum Language {
            $( $language, )*
        }

        impl Language {
            pub fn all() -> Vec<Language> {
                vec![
                    $( Language::$language, )*
                ]
            }
        }
    }
}

language_enum!([DE, EN, ES, FR, KO]);

impl FromStr for Language {
    type Err = String;
    fn from_str(it: &str) -> Result<Language, Self::Err> {
        match &*it.to_lowercase() {
            "en" => Ok(Language::EN),
            "fr" => Ok(Language::FR),
            "es" => Ok(Language::ES),
            "ko" => Ok(Language::KO),
            "de" => Ok(Language::DE),
            _ => Err(format!("Unknown language {}", it)),
        }
    }
}

impl ToString for Language {
    fn to_string(&self) -> String {
        match *self {
            Language::EN => "en".to_string(),
            Language::FR => "fr".to_string(),
            Language::ES => "es".to_string(),
            Language::KO => "ko".to_string(),
            Language::DE => "de".to_string(),
        }
    }
}

impl Language {
    pub fn punctuation(&self) -> &'static str {
        match self {
            _ => PUNCTUATION
        }
    }

    pub fn default_separator(&self) -> &'static str {
        match self {
            _ => SPACE
        }
    }
}
