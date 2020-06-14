use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;

pub mod event;
pub mod gfx;
pub mod level;
pub mod tile;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum RawCommand {
	_KeyPress(Keycode),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
	pub command_queue:				VecDeque<RawCommand>,
	pub level:							Vec<level::Level>,
	pub x:								usize,
	pub y:								usize,
	pub n:								usize,
	pub screen_size_x:					usize,
	pub screen_size_y:					usize,
}

impl Cave {
	pub fn new(screen_x: usize, screen_y: usize) -> Self {
		let command_queue = VecDeque::new();
		let level: Vec<level::Level> = Vec::new();
		let (screen_size_x, screen_size_y) = (screen_x / 32usize, screen_y / 32usize);
		
		Cave {
			command_queue,
			level,
			x: 0,
			y: 0,
			n: 0,
			screen_size_x,
			screen_size_y,
		}
	}

	pub fn generate(&mut self, width: usize, height: usize, levels: usize) {
		println!("generate width = {}, height = {}, levels = {}", width, height, levels);

		for i in 0..levels {
			self.level.push(level::Level::new());
			self.level[i].init(width, height, i);
		}

		let index = 3 * width + 10;
		self.level[0].grid[index] = 10;
	}

	pub fn process_event(&mut self, event: Event) -> bool {
		match event {
			Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
				return false;
			},
			Event::Quit { .. } => {
				return false;
			},
			Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
				println!("keypress: down");
				event::move_down(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
				println!("keypress: left");
				event::move_left(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
				println!("keypress:  up");
				event::move_up(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
				println!("keypress: right");
				event::move_right(self);
			},
			_ => {}
		}

		println!("focus x = {}, y = {}, n = {}", self.x, self.y, self.n);

		true
	}
}
