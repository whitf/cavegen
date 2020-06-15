



pub struct Point {
	pub x:							usize,
	pub y:							usize,
	pub z:							usize,
}

impl Point {
	pub fn new(x: usize, y: usize, z: usize) -> Self {
		Point {
			x,
			y,
			z,
		}
	}
}

pub struct Cell {
	pub p:							Point,
}