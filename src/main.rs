mod config;
mod hyprland;

use hyprland::rules::Rule;
use hyprland::rules::StaticRule;
use std::env;

fn apply_static_rules(ipc: &hyprland::send::Hyprctl, rules: &Vec<StaticRule>) {
    match ipc.get_active_window() {
        Some(window) => {
            rules.iter().for_each(|rule| {
                rule.apply_properties(&ipc, &window);
            });
        }
        None => (),
    }
}

fn main() {
    let static_rules = match env::args().nth(1) {
        Some(arg) => config::load_config(&arg),
        None => {
            let home = env::var("HOME").unwrap();
            let config = format!("{}/.config/hypr/rules.yaml", home);
            config::load_config(&config)
        }
    };

    match static_rules {
        Ok(rules) => {
            if !rules.is_empty() {
                let mut events = hyprland::events::Hyprctl::new();
                let ipc = hyprland::send::Hyprctl::new();
                events.subscribe(hyprland::events::Event::ActiveWindowV2, move |_data| {
                    apply_static_rules(&ipc, &rules);
                });
                events.listen();
            }
        }
        Err(config::LoadError::Io(e)) => eprintln!("Error reading file. {e}"),
        Err(config::LoadError::Yaml(e)) => eprintln!("Error parsing YAML. {e}"),
        Err(config::LoadError::InvalidConfig) => eprintln!("Rules aren't an array"),
    }
}
