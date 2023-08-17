use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub struct Input {
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            is_up_pressed: false,
            is_down_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn is_movement_keys_pressed(&self) -> bool {
        self.is_down_pressed
            || self.is_up_pressed
            || self.is_right_pressed
            || self.is_left_pressed
    }

    /// Horizontal movement unit-vector based on pressed keys
    pub fn y(&self) -> i32 {
        if self.is_up_pressed && !self.is_down_pressed {
            return -1;
        } else if self.is_down_pressed && !self.is_up_pressed {
            return 1;
        }
        0
    }

    /// Vertical movement unit-vector based on pressed keys
    pub fn x(&self) -> i32 {
        if self.is_left_pressed && !self.is_right_pressed {
            return -1;
        } else if self.is_right_pressed && !self.is_left_pressed {
            return 1;
        }
        0
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
