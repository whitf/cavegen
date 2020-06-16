use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::VecDeque;

use crate::episu;

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

		//let mut engine: episu::Engine = episu::Engine::new();

		for i in 0..levels {
			self.level.push(level::Level::new());
			self.level[i].init(width, height, i);

			let mut engine: episu::Engine = episu::Engine::new(width, height);

			self.level[i].grid = engine.generate(
				episu::Style::Cave,
				episu::Point::new(15usize, 15usize, i as u8)
			);
		}

		/*
		self.level[0].grid[1 * width + 1] = 10;									// top left
		self.level[0].grid[1 * width + 2] = 10;					
		self.level[0].grid[2 * width + 1] = 10;

		self.level[0].grid[1 * width + (width - 2)] = 10;						// top right
		self.level[0].grid[1 * width + (width - 3)] = 10;
		self.level[0].grid[2 * width + (width - 2)] = 10;
		
		self.level[0].grid[(height - 2) * width + 1] = 10;						// bottom left
		self.level[0].grid[(height - 2) * width + 2] = 10;
		self.level[0].grid[(height - 3) * width + 1] = 10;
		
		self.level[0].grid[(height - 2) * width + (width - 2)] = 10;			// bottom right
		self.level[0].grid[(height - 2) * width + (width - 3)] = 10;
		self.level[0].grid[(height - 3) * width + (width - 2)] = 10;
		*/

		//let index = 3 * width + 10;
		//self.level[0].grid[index] = 10;
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
				event::move_down(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
				event::move_left(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
				event::move_up(self);
			},
			Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
				event::move_right(self);
			},
			_ => {}
		}

		//println!("focus x = {}, y = {}, n = {}", self.x, self.y, self.n);
		true
	}
}