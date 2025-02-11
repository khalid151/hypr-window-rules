use super::Window;
use std::{
    env,
    io::{Read, Write},
    os::unix::net::UnixStream,
    time::Duration,
};
use yaml_rust::YamlLoader;

#[allow(dead_code)]
pub enum NotifyIcon {
    Warning = 0,
    Info = 1,
    Hint = 2,
    Error = 3,
    Confused = 4,
    Ok = 5,
    None = -1,
}

pub struct Hyprctl {
    socket_path: String,
}

#[allow(dead_code)]
impl Hyprctl {
    pub fn new() -> Self {
        let xdg_runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set.");
        let hyprland_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE is not set.");
        let socket_path = format!("{xdg_runtime}/hypr/{hyprland_instance}/.socket.sock");
        Hyprctl { socket_path }
    }

    fn send(&self, message: &str) -> Option<String> {
        let stream = UnixStream::connect(&self.socket_path);
        match stream {
            Ok(mut socket) => {
                if let Err(e) = socket.write_all(message.as_bytes()) {
                    eprintln!("Couldn't write to socket. {}", e);
                } else {
                    let mut buf = Vec::new();
                    let _ = socket.read_to_end(&mut buf);
                    let data = String::from_utf8_lossy(&buf).to_string();
                    return Some(data);
                }
            }
            Err(e) => eprintln!("Error connecting to socket. {}", e),
        }
        None
    }

    pub fn notify(&self, icon: NotifyIcon, time: Duration, color: &str, message: &str) {
        let msg = format!(
            "notify {} {} {color} {message}",
            icon as isize,
            time.as_millis()
        );
        self.send(&msg);
    }

    pub fn dispatch(&self, msg: &str) {
        self.send(&format!("dispatch {msg}"));
    }

    pub fn get_active_window(&self) -> Option<Window> {
        match self.send("activewindow") {
            Some(window) => {
                if window == "Invalid" {
                    return None;
                }

                let cleand_up_response = window
                    .lines()
                    .skip(1)
                    .map(|line| line.trim_start())
                    .collect::<Vec<_>>()
                    .join("\n");
                let address = {
                    let first_line = window.lines().next().unwrap();
                    let start_i = first_line.find("Window ").unwrap() + "Window ".len();
                    let end_i = first_line.find(" ->").unwrap();
                    isize::from_str_radix(&first_line[start_i..end_i], 16).unwrap()
                };

                match YamlLoader::load_from_str(&cleand_up_response) {
                    Ok(window) => {
                        let window = Window {
                            address,
                            title: String::from(window[0]["title"].as_str().unwrap()),
                            class: String::from(window[0]["class"].as_str().unwrap()),
                        };
                        Some(window)
                    }
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}
