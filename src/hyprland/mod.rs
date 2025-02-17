pub mod events;
pub mod rules;
pub mod send;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Window {
    pub address: isize,
    pub class: String,
    pub title: String,
}
