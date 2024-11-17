use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

#[derive(Default, Debug)]
pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    /// Horizontal movement unit-vector based on pressed keys
    pub fn y(&self) -> i32 {
        if self.up && !self.down {
            return -1;
        }
        if self.down && !self.up {
            return 1;
        }
        0
    }

    /// Vertical movement unit-vector based on pressed keys
    pub fn x(&self) -> i32 {
        if self.left && !self.right {
            return -1;
        }
        if self.right && !self.left {
            return 1;
        }
        0
    }

    pub fn none(&self) -> bool {
        !self.up && !self.down && !self.left && !self.right
    }

    pub fn vector(&self) -> (i32, i32) {
        (self.x(), self.y())
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.up = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.left = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.down = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.right = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
