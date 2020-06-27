
#[derive(Debug, Eq, PartialEq)]
pub struct Portal {
	pub tile:				u8,
	pub x:					u8,
	pub y:					u8,
	pub target_level:		u8,
	pub target_x:			u8,
	pub target_y:			u8,
}

impl Portal {
	pub fn new() -> Self {
		Portal {
			tile: 0,
			x: 0,
			y: 0,
			target_level: 0,
			target_x: 0,
			target_y: 0,
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub struct Level {
	pub id:				u8,
	pub grid:			Vec<u8>,
	pub map_size_x:		usize,
	pub map_size_y:		usize,
	pub portal:			Vec<Portal>,
}

impl Level {
	pub fn new() -> Self {
		Level {
			id: 0,
			grid: Vec::new(),
			map_size_x: 0,
			map_size_y: 0,
			portal: Vec::new(),
		}
	}

	pub fn init(&mut self, width: usize, height: usize, n: usize) {
		self.id = n as u8;
		self.map_size_x = width;
		self.map_size_y = height;

		println!("[level] init (width, height) = ({}, {})", width, height);

		for _ in 0..(self.map_size_x * self.map_size_y) {
			self.grid.push(0u8);
		}
	}

    #[allow(dead_code)]
	fn index(&mut self, x: i32, y: i32) -> usize {
		let index = (y * self.map_size_x as i32) + x;

		index as usize
	}
}
