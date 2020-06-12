use sdl2::rect::Rect;

#[derive(Debug)]
pub enum TileType {
	Void,
	Grass,
	SandFloor1,	
}

#[derive(Debug)]
pub struct Tile {
	raw_type:				u8,
	tile_type:				TileType,
}


impl Tile {
	pub fn new() -> Self {
		Tile {
			raw_type: 0,
			tile_type: TileType::Void,
		}
	}
}

pub fn get_rect(raw_type: u8) -> Rect {
	let tile_res = 32u32;
	let (x, y) = find_texture(raw_type);

	Rect::new(
		x * tile_res as i32,
		y * tile_res as i32,
		tile_res,
		tile_res,
	)
}

pub fn get_type(raw_type: u8) -> TileType {
	match raw_type {
		0 	=> TileType::Void,
		10 	=> TileType::SandFloor1,
		60	=> TileType::Grass,
		_ 	=> TileType::Void,
	}
}

fn find_texture(raw_type: u8) -> (i32, i32) {
	match raw_type {
		0		=>	(18, 0),		// void
		10		=> 	(2, 4),		// sandy floor
		50		=>	(27, 3),	// grassy floor
		_		=>	(1, 0),		// default (void)
	}
}
