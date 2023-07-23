use std::collections::HashMap;
use winit;

#[derive(Debug)]
pub struct Keymap {
    pub keys: HashMap<winit::event::VirtualKeyCode, winit::event::ElementState>
}

impl Keymap {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }
}

