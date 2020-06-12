use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;

pub mod gfx;
pub mod tile;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum RawCommand {
	_KeyPress(Keycode),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
	pub command_queue:				VecDeque<RawCommand>,
}

impl Cave {
	pub fn new() -> Self {
		let command_queue = VecDeque::new();

		Cave{
			command_queue,
		}
	}

	pub fn process_event(&mut self, event: Event) -> bool {
		match event {
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return false;
			},
			Event::Quit { .. } => {
				return false;
			},
			_ => {}
		}

		true
	}
}
