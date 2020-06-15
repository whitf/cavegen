

#[derive(Debug)]
pub enum LevelType {
	Cave,
	Dessert,
	Dungeon,
	Grasslands,
	Hills,
	Forest,
	ForestHeavy,
	ForestSparse,
	Marsh,
	Mountain,
	Tundra,
	Village,
}


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
		for _ in 0..(width * height) {
			self.grid.push(0);
		}
	}

	fn index(&mut self, x: i32, y: i32) -> usize {
		let index = (y * self.map_size_x as i32) + x;

		index as usize
	}

	pub fn generate(&mut self, level_type: LevelType) -> bool {
		let (mut start_x, mut start_y) = (0i32, 0i32);
		let (mut end_x, mut end_y) = (self.map_size_x as i32, self.map_size_y as i32);

		println!("[level::generate] generate from ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);

		// "shrink box to maintain a void "buffer" around the outside.
		start_x += 1;
		start_y += 1;
		end_x -= 1;
		end_y -= 1;

		// Two basic generation methods:
		// "open" generation for fields, forests, etc.
		// "closed" generation for caves, dungeons, etc.

		if !self.generate_open(level_type, start_x, end_x, start_y, end_y) {
			return false;
		}

		println!("[level::generate] ended");
		true
	}

	fn generate_open(&mut self, level_type: LevelType, start_x: i32, end_x: i32, start_y: i32, end_y: i32) -> bool {
		println!("[level::generate_open] started");

		let (mut start_x, mut end_x, mut start_y, mut end_y) = (start_x, end_x, start_y, end_y);

		for x in start_x..end_x {
			self.grid[(start_y as usize * self.map_size_x) + x as usize] = 100u8;
			self.grid[((end_y - 1) as usize) * self.map_size_x + x as usize] = 100u8;
		}

		for y in start_y..end_y {
			self.grid[(y as usize * self.map_size_x) + start_x as usize] = 100u8;
			self.grid[(y as usize * self.map_size_x) + (end_x - 1) as usize] = 100u8;
		}

		// "shrink" map box again.
		start_x += 1;
		start_y += 1;
		end_x -= 1;
		end_y -= 1;

		for y in start_y..end_y {
			for x in start_x..end_x {
				self.grid[(y as usize * self.map_size_x) + x as usize] = 10u8;
			}
		}

		println!("[level::generate_open] ended");
		true
	}

	fn generate_closed(&mut self, level_type: LevelType, start_x: i32, end_x: i32, start_y: i32, end_y: i32) -> bool {
		println!("[level::generate_closed] ended");
		
		let (mut start_x, mut end_x, mut start_y, mut end_y) = (start_x, end_x, start_y, end_y);
		



		println!("[level::generate_closed] ended");
		true
	}
}
