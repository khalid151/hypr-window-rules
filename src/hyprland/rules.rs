use yaml_rust::Yaml;

fn yaml_to_string(yaml: &Yaml) -> String {
    match yaml {
        Yaml::String(s) => s.to_string(),
        Yaml::Real(r) => r.to_string(),
        Yaml::Integer(i) => i.to_string(),
        Yaml::Boolean(b) => {
            if *b {
                String::from("1")
            } else {
                String::from("0")
            }
        }
        _ => String::new(),
    }
}

fn process_match(match_rules: &Yaml) -> String {
    let mut final_match: String = String::from("");

    let keys = match_rules.as_hash().unwrap().keys();
    for field in keys {
        let field = field.as_str().unwrap();
        if !(field == "dynamic") {
            let current_match = format!("{}:{},", field, yaml_to_string(&match_rules[field]));
            final_match.push_str(&current_match);
        }
    }
    // remove the trailing comma
    final_match.remove(final_match.len() - 1);
    final_match
}

fn process_properties(properties: &Yaml) -> Vec<String> {
    let mut final_properties: Vec<String> = Vec::new();
    let keys = properties.as_hash().unwrap().keys();

    for field in keys {
        let field = field.as_str().unwrap();
        match &properties[field] {
            Yaml::String(s) => match field {
                "plugin" => final_properties.push(format!("{}:{}", field, s)),
                _ => final_properties.push(format!("{} {}", field, s)),
            },
            Yaml::Integer(i) => final_properties.push(format!("{} {}", field, i)),
            Yaml::Boolean(b) => {
                if *b {
                    final_properties.push(format!("{}", field));
                } else {
                    match field {
                        "allowsinput" => final_properties.push("allowsinput 0".to_string()),
                        "dimaround" => final_properties.push("nodim".to_string()),
                        _ => final_properties.push(format!("{} 0", field)),
                    }
                }
            }
            _ => (),
        }
    }

    final_properties
}

#[allow(dead_code)]
pub struct Rule {
    match_rules: String,
    properties: Vec<String>,
    follow_title: bool,
}

impl Rule {
    pub fn new(match_rules: &Yaml, properties: &Yaml) -> Self {
        let follow_title = {
            let follow = match_rules["follow-title"].as_bool();
            if let None = follow {
                false
            } else {
                follow.unwrap()
            }
        };
        let match_rules = process_match(&match_rules);
        let properties = process_properties(&properties);
        Rule {
            match_rules,
            properties,
            follow_title,
        }
    }

    pub fn compile(&self) -> Vec<String> {
        let mut rules: Vec<String> = Vec::new();
        for property in &self.properties {
            rules.push(format!("{},{}", property, self.match_rules));
        }
        rules
    }
}
