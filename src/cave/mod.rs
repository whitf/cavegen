use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;

pub mod gfx;
pub mod tile;
pub mod level;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum RawCommand {
	_KeyPress(Keycode),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
	pub command_queue:				VecDeque<RawCommand>,
	level:							Vec<level::Level>,
	x:								usize,
	y:								usize,
	n:								usize,
}

impl Cave {
	pub fn new() -> Self {
		let command_queue = VecDeque::new();
		let level: Vec<level::Level> = Vec::new();

		Cave{
			command_queue,
			level,
			x: 0,
			y: 0,
			n: 0,
		}
	}

	pub fn generate(&mut self, width: usize, height: usize, levels: usize) {

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
