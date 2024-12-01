use std::collections::VecDeque;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::movement_util::Direction;

#[derive(Default, Debug)]
pub struct Input {
    direction_stack: VecDeque<Direction>,
    shift: bool,
    j: bool,
    k: bool,
    space: bool,
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
                        self.handle_direction_key(Direction::Up, is_pressed);
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.handle_direction_key(Direction::Left, is_pressed);
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.handle_direction_key(Direction::Down, is_pressed);
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.handle_direction_key(Direction::Right, is_pressed);
                        true
                    }
                    VirtualKeyCode::J => {
                        self.j = is_pressed;
                        true
                    }
                    VirtualKeyCode::K => {
                        self.k = is_pressed;
                        true
                    }
                    VirtualKeyCode::Space => {
                        self.space = is_pressed;
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

    fn handle_direction_key(&mut self, direction: Direction, is_pressed: bool) {
        if is_pressed {
            // Remove any existing instance of this direction
            self.direction_stack.retain(|&d| d != direction);
            // Push to back (top of stack)
            self.direction_stack.push_back(direction);
        } else {
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
        self.shift = false;
    }

    pub fn j(&self) -> bool {
        self.j
    }

    pub fn k(&self) -> bool {
        self.k
    }

    pub fn space(&self) -> bool {
        self.space
    }
}
