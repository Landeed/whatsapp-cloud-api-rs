use serde::{Deserialize, Serialize};

use super::{component::Component, Language};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub language: Language,
    pub components: Option<Vec<Component>>,
}

impl Template {
    pub fn new(name: &str, language: &str) -> Self {
        let language = Language::new(language);
        Self {
            name: name.into(),
            language,
            components: None,
        }
    }

    pub fn with_components(name: &str, language: &str, components: Vec<Component>) -> Self {
        let language = Language::new(language);
        Self {
            name: name.into(),
            language,
            components: Some(components),
        }
    }
}

