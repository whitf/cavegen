use rand;
use rand::Rng;

pub fn generate(engine: &mut super::Engine, start: super::Point, cap_x: usize, cap_y: usize) -> Vec<u8> {
	let mut grid = engine.init();

	let (mut start_x, mut start_y) = (0usize, 0usize);
	let (mut end_x, mut end_y) = (cap_x, cap_y);

	println!("[cave::generate] generate from ({}, {}) to ({}, {})", start_x, start_y, end_x, end_y);

	// "shrink box to maintain a void "buffer" around the outside.
	start_x += 1;
	start_y += 1;
	end_x -= 1;
	end_y -= 1;

	/*

	for x in start_x..end_x {
		grid[(start_y * cap_x) + x] = 100u8;
		grid[(end_y - 1) * cap_x + x] = 100u8;
	}

	for y in start_y..end_y {
		grid[(y * cap_x) + start_x] = 100u8;
		grid[(y * cap_x) + (end_x - 1)] = 100u8;
	}
	*/

	// // "shrink" map box again.
	// start_x += 1;
	// start_y += 1;
	// end_x -= 1;
	// end_y -= 1;

	/*
	for y in start_y..end_y {
		for x in start_x..end_x {
			grid[(y * cap_x) + x] = 10u8;
		}
	}
	*/

	let mut rng = rand::thread_rng();
	let mut x = start.x;
	let mut y = start.y;

	grid[(y * cap_x) + x] = 10u8;

	for _ in 0..((cap_x as f32 * cap_y as f32 * 1.5) as u32) {
		let mut dir: super::Direction = rng.gen::<super::Direction>();
		println!("Direction = {:?}", dir);

		let x_y = engine.move_to(x, y, dir);

		//(x, y) = engine.move_to(x, y, dir);
		x = x_y.0;
		y = x_y.1;

		grid[(y * cap_x) + x] = 10u8;
	}

	// Fill "void" tiles with stone - creates an outer wall and "rocks".
	for j in start_y..end_y {
		for i in start_x..end_x {
			if grid[(j * cap_x) + i] == 0u8 {
				grid[(j * cap_x) + i] = 100u8;
			}
		}
	}

	println!("[cave::generate] returning grid with {} tiles.", grid.len());
	return grid;
}
