use sdl2::rect::Rect;
use sdl2::render::{Texture, WindowCanvas};

use crate::cave;
use crate::cave::tile;
use crate::cave::level;

pub struct CaveBox {
	rect:				Rect,
}

impl CaveBox {
	pub fn new(rect: Rect) -> Self {
		CaveBox {
			rect,
		}
	}

	pub fn draw(&mut self, texture: &Texture, cave: &cave::Cave, canvas: &mut WindowCanvas) {
		let (width, height) = (self.rect.width(), self.rect.height());

		let u_res = 64u32;
		let i_res = 64i32;
		let tile_res = 32u32;

		let screen_size_x: usize = (width/u_res) as usize;
		let screen_size_y: usize = (height/u_res) as usize;
		let screen_cap_x: i32 = (width/tile_res) as i32;
		let screen_cap_y: i32 = (height/tile_res) as i32;

		// Fill box with void tiles.
		let void_tile = tile::get_rect(0);
		for y in 0..screen_cap_x {
			for x in 0..screen_cap_y {
				canvas.copy(&texture, void_tile, Rect::new(x * i_res, y * i_res, u_res, u_res))
					.expect("Failed to copy initial coid tile(s).");
			}
		}

		let map_size_x: usize = cave.level[cave.n].map_size_x;
		let map_size_y: usize = cave.level[cave.n].map_size_y;

		let mut map_dist: Vec<usize> = Vec::new();
		map_dist.push(cave.y);
		map_dist.push(map_size_x - cave.x);
		map_dist.push(map_size_y - cave.y);
		map_dist.push(cave.x);

		let mut screen_dist: Vec<usize> = Vec::new();
		screen_dist.push(screen_size_y / 2usize);
		screen_dist.push(screen_size_x / 2usize);
		screen_dist.push(screen_size_y / 2usize);
		screen_dist.push(screen_size_x / 2usize);

		for i in 0..map_dist.len() {
			if screen_dist[i] < map_dist[i] {
				map_dist[i] = screen_dist[i];
			}
		}

		
		let y_start = cave.y - map_dist[0];
		let x_start = cave.x - map_dist[3];
		let x_end = x_start + screen_size_x;
		let y_end = y_start + screen_size_y;

		let level = &cave.level[ cave.n ];

		// copy "background" tiles from this level.
		for y in y_start..y_end {
			if y < map_size_y {
				for x in x_start..x_end {
					if x < map_size_x {
						let index = (y * map_size_x) + x;
						let my_tile = tile::get_rect(level.grid[index]);

						// Copy tile to the "screen map" on the canvas.
						// using x - x_start and y - y_start translate "moved" x and y on the tile map to the "static" x and y on the screen.
						canvas.copy(&texture, my_tile, Rect::new(((x - x_start) as i32) * i_res, ((y - y_start) as i32) * i_res, u_res, u_res))
							.expect("Texture Copy Error - Could not copy background tile(s).");
					}
				}
			}
		}

	}

}
