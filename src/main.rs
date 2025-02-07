mod hyprland;

use std::env;
use std::process::exit;

use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

use hyprland::rules::Rule;

fn exit_with_message(message: &str) {
    eprintln!("{message}");
    exit(1);
}

fn compile_rule(rule: &Rule, print_to_stdout: bool) {
    if print_to_stdout {
        for line in rule.compile() {
            println!("windowrulev2 = {}", line);
        }
    }
}

fn apply_config(yaml: &str, print_to_stdout: bool) {
    match YamlLoader::load_from_str(yaml) {
        Ok(config) => {
            let rules = &config[0].as_vec().expect("Invalid config format.");
            for rule in *rules {
                let props_yaml = &rule["properties"];

                match &rule["match"] {
                    Yaml::Array(arr) => {
                        for element in arr {
                            let rule = Rule::new(element, props_yaml);
                            compile_rule(&rule, print_to_stdout)
                        }
                    }
                    Yaml::Hash(_) => {
                        let rule = Rule::new(&rule["match"], props_yaml);
                        compile_rule(&rule, print_to_stdout)
                    }
                    _ => (),
                }
            }
        }
        Err(_) => exit_with_message("Invalid yaml file."),
    }
}

fn load_config(path: &str, print_to_stdout: bool) {
    match std::fs::read_to_string(path) {
        Ok(yaml) => apply_config(&yaml, print_to_stdout),
        Err(_) => exit_with_message("Couldn't read file."),
    }
}

fn main() {
    match env::args().nth(1) {
        Some(arg) => {
            load_config(&arg, true);
        }
        None => {
            // Try to load from default location
            let home = env::var("HOME").unwrap();
            let config = format!("{}/.config/hypr/rules.yaml", home);
            load_config(&config, true);
        }
    }
}
