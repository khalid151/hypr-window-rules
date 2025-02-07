use std::{
    collections::HashMap,
    env,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
};

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Event {
    ActiveWindow,
    ActiveWindowV2,
    ConfigReloaded,
    None,
}

impl Event {
    fn from_str(event_name: &str) -> Event {
        match event_name {
            "activewindow" => Event::ActiveWindow,
            "activewindowv2" => Event::ActiveWindowV2,
            "configreloaded" => Event::ConfigReloaded,
            _ => Event::None,
        }
    }
}

pub struct Hyprctl {
    socket: UnixStream,
    callbacks: HashMap<Event, Vec<fn(&str)>>,
}

#[allow(dead_code)]
impl Hyprctl {
    pub fn new() -> Self {
        let xdg_runtime = env::var("XDG_RUNTIME_DIR").expect("XDG_RUNTIME_DIR is not set.");
        let hyprland_instance = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .expect("HYPRLAND_INSTANCE_SIGNATURE is not set.");
        let socket_path = format!("{xdg_runtime}/hypr/{hyprland_instance}/.socket2.sock");
        let socket = UnixStream::connect(socket_path).expect("Couldn't connect to .socket2.sock");
        Hyprctl {
            socket,
            callbacks: HashMap::new(),
        }
    }

    pub fn listen(&self) {
        let reader = BufReader::new(&self.socket);
        for line in reader.lines() {
            match line {
                Ok(event) => self.process_events(&event),
                Err(_) => {
                    eprintln!("Error reading from socket");
                    break;
                }
            }
        }
    }

    pub fn subscribe(&mut self, event: Event, callback: fn(&str)) {
        if !self.callbacks.contains_key(&event) {
            let mut new_vec: Vec<fn(&str)> = Vec::new();
            new_vec.push(callback);
            self.callbacks.insert(event, new_vec);
        } else {
            if let Some(callbacks) = self.callbacks.get_mut(&event) {
                callbacks.push(callback);
            }
        }
    }

    fn process_events(&self, event: &str) {
        if let Some((name, data)) = event.split_once(">>") {
            if let Some(callbacks) = self.callbacks.get(&Event::from_str(&name)) {
                for callback in callbacks {
                    callback(data);
                }
            }
        }
    }
}
