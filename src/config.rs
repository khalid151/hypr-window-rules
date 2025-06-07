use crate::Rule;
use crate::StaticRule;

use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

#[derive(Debug)]
pub enum LoadError {
    Io(String),
    Yaml(String),
    InvalidConfig,
}

impl LoadError {
    pub fn from_io(e: std::io::Error) -> Self {
        LoadError::Io(e.to_string())
    }

    pub fn from_yaml(e: yaml_rust::ScanError) -> Self {
        LoadError::Yaml(e.to_string())
    }
}

pub fn load_config(path: &str) -> Result<Vec<StaticRule>, LoadError> {
    let file = std::fs::read_to_string(path).map_err(LoadError::from_io)?;
    let config = YamlLoader::load_from_str(&file).map_err(LoadError::from_yaml)?;

    let rules = config
        .first()
        .unwrap()
        .as_vec()
        .ok_or(LoadError::InvalidConfig)?
        .iter()
        .flat_map(|rule_block| {
            let properties = &rule_block["properties"];
            let match_entry = &rule_block["match"];

            let rule_iter = match match_entry {
                Yaml::Array(a) => a
                    .iter()
                    .map(move |m| Rule::new(m, properties))
                    .collect::<Vec<_>>()
                    .into_iter(),
                Yaml::Hash(_) => vec![Rule::new(match_entry, properties)].into_iter(),
                _ => vec![].into_iter(),
            };

            rule_iter.filter_map(|mut rule| {
                rule.compile()
                    .iter()
                    .for_each(|l| println!("windowrule = {l}"));
                rule.static_properties.take()
            })
        })
        .collect();

    Ok(rules)
}
