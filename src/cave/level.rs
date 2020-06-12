



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
}
