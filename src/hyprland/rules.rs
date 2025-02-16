use yaml_rust::Yaml;

use super::Window;

pub enum StaticRules {
    Float,
    Tile,
    Fullscreen,
    Maximize,
    Move,
    Size,
    Center,
    Workspace,
    Pin,
    None,
}

impl StaticRules {
    fn from_str(name: &str) -> StaticRules {
        match name {
            "float" => StaticRules::Float,
            "tile" => StaticRules::Tile,
            "fullscreen" => StaticRules::Fullscreen,
            "maximize" => StaticRules::Maximize,
            "move" => StaticRules::Move,
            "size" => StaticRules::Size,
            "center" => StaticRules::Center,
            "workspace" => StaticRules::Workspace,
            "pin" => StaticRules::Pin,
            _ => StaticRules::None,
        }
    }

    fn command_from_str(name: &str) -> Option<String> {
        let rule = StaticRules::from_str(&name);
        match rule {
            StaticRules::Pin | StaticRules::Fullscreen => Some(String::from(name)),
            StaticRules::Maximize => Some(String::from("fullscreen 1")),
            StaticRules::Float => Some(String::from("setfloating")),
            StaticRules::Tile => Some(String::from("settiled")),
            StaticRules::Center => Some(format!("{name}window")),
            StaticRules::Workspace => Some(String::from("movetoworkspace")),
            StaticRules::Move => Some(String::from("movewindowpixel exact")),
            StaticRules::Size => Some(String::from("resizewindowpixel exact")),
            StaticRules::None => None,
        }
    }
}

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

fn process_match(match_rules: &Yaml) -> (String, Option<String>, Option<String>) {
    let mut title: Option<String> = None;
    let mut class: Option<String> = None;

    let final_match = match_rules
        .as_hash()
        .unwrap()
        .keys()
        .filter_map(|key| key.as_str())
        .filter(|&f| f != "follow-title")
        .map(|field| {
            let value = yaml_to_string(&match_rules[field]);

            match field {
                "class" => class = Some(value.clone()),
                "title" => title = Some(value.clone()),
                _ => (),
            }

            format!("{}:{}", field, value)
        })
        .collect::<Vec<_>>()
        .join(",");

    (final_match, title, class)
}

fn process_properties(properties: &Yaml) -> (Vec<String>, Vec<String>) {
    let (all_props, static_props) = properties.as_hash().unwrap().iter().fold(
        (Vec::new(), Vec::new()),
        |(mut all_props, mut static_props), (key, value)| {
            let field = match key.as_str() {
                Some(f) => f,
                None => return (all_props, static_props),
            };

            all_props.push(match value {
                Yaml::String(s) => handle_property_field(field, s),
                Yaml::Integer(i) => format!("{} {}", field, i),
                Yaml::Real(r) => format!("{} {}", field, r),
                Yaml::Boolean(b) => handle_bool_property(field, *b),
                _ => return (all_props, static_props),
            });

            if let Some(static_prop) = StaticRules::command_from_str(field) {
                static_props.push(match StaticRules::from_str(field) {
                    StaticRules::Move | StaticRules::Size | StaticRules::Workspace => match value {
                        Yaml::String(s) => format!("{static_prop} {s},"),
                        Yaml::Integer(i) => format!("{static_prop} {i},"),
                        _ => return (all_props, static_props),
                    },
                    _ => format!("{static_prop} "),
                });
            }

            (all_props, static_props)
        },
    );

    (all_props, static_props)
}

fn handle_property_field(field: &str, param: &str) -> String {
    match field {
        "plugin" => format!("{}:{}", field, param),
        _ => format!("{} {}", field, param),
    }
}

fn handle_bool_property(field: &str, b: bool) -> String {
    if b {
        field.to_string()
    } else {
        match field {
            "dimaround" => "nodim".into(),
            _ => format!("{field} 0"),
        }
    }
}

#[derive(Debug)]
pub struct StaticRule {
    title: String,
    class: String,
    properties: Vec<String>,
}

impl StaticRule {
    pub fn apply_properties(&self, ipc: &super::send::Hyprctl, window: &Window) {
        if self.title == window.title && self.class == window.class {
            self.properties.iter().for_each(|p| {
                let compiled_property = format!("{p}address:0x{:x}", window.address);
                ipc.dispatch(&compiled_property);
            });
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    match_rules: String,
    properties: Vec<String>,
    pub static_properties: Option<StaticRule>,
}

impl Rule {
    pub fn new(match_rules: &Yaml, properties: &Yaml) -> Self {
        let follow_title = match match_rules["follow-title"] {
            Yaml::Boolean(b) => b,
            _ => false,
        };

        let (match_rules, title, class) = process_match(&match_rules);
        let (properties, static_props) = process_properties(&properties);

        let static_properties: Option<StaticRule> = {
            if follow_title {
                let (title, class) = match (title, class) {
                    (Some(t), Some(c)) => (t, c),
                    _ => {
                        eprintln!("Error: follow-title requires both title and class to be set.");
                        std::process::exit(1);
                    }
                };

                Some(StaticRule {
                    title: title.to_string(),
                    class: class.to_string(),
                    properties: static_props,
                })
            } else {
                None
            }
        };

        Rule {
            match_rules,
            properties,
            static_properties,
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
