use std::collections::{HashSet, VecDeque};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::movement_util::Direction;

#[derive(Default, Debug)]
pub struct Input {
    direction_stack: VecDeque<Direction>,
    pressed_keys: HashSet<VirtualKeyCode>,
    shift: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
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
                        self.handle_direction_key(*keycode, Direction::Up, is_pressed);
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.handle_direction_key(*keycode, Direction::Left, is_pressed);
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.handle_direction_key(*keycode, Direction::Down, is_pressed);
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.handle_direction_key(*keycode, Direction::Right, is_pressed);
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.shift = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn handle_direction_key(
        &mut self,
        key: VirtualKeyCode,
        direction: Direction,
        is_pressed: bool,
    ) {
        if is_pressed {
            if !self.pressed_keys.contains(&key) {
                self.pressed_keys.insert(key);
                // Remove any existing instance of this direction
                self.direction_stack.retain(|&d| d != direction);
                // Push to back (top of stack)
                self.direction_stack.push_back(direction);
            }
        } else {
            self.pressed_keys.remove(&key);
            self.direction_stack.retain(|&d| d != direction);
        }
    }

    pub fn current_direction(&self) -> Option<Direction> {
        self.direction_stack.back().copied()
    }

    pub fn shift(&self) -> bool {
        self.shift
    }

    pub fn none(&self) -> bool {
        self.direction_stack.is_empty()
    }

    pub fn x(&self) -> i32 {
        match self.current_direction() {
            Some(Direction::Left) => -1,
            Some(Direction::Right) => 1,
            _ => 0,
        }
    }

    pub fn y(&self) -> i32 {
        match self.current_direction() {
            Some(Direction::Up) => -1,
            Some(Direction::Down) => 1,
            _ => 0,
        }
    }

    pub fn vector(&self) -> (i32, i32) {
        (self.x(), self.y())
    }

    pub fn clear(&mut self) {
        self.direction_stack.clear();
        self.pressed_keys.clear();
        self.shift = false;
    }
}
