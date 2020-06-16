//use rand::{Rand, Rng};
use rand_derive::Rand;
use uuid::Uuid;

pub mod cave;

#[derive(Debug)]
pub enum Style {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Rand)]
pub enum Direction {
	North,
	East,
	South,
	West,
}

pub struct Point {
	x:					usize,
	y:					usize,
	n:					u8,
}

impl Point {
	pub fn new(x: usize, y: usize, n: u8) -> Self {
		Point {
			x,
			y,
			n,
		}
	}
}

pub struct Engine {
	pub cap_x:						usize,
	pub cap_y:						usize,
	pub id:							Uuid,
}

impl Engine {
	pub fn new(cap_x: usize, cap_y: usize) -> Self {
		let id = Uuid::new_v4();

		Engine {
			cap_x,
			cap_y,
			id,
		}
	}

	pub fn generate(&mut self, style: Style, start: Point) -> Vec<u8> {
		//let mut grid: Vec<u8> = Vec::with_capacity(cap_x * cap_y);


		match style {
			Style::Cave => {
				return cave::generate(self, start, self.cap_x, self.cap_y);
			},
			_ => {}
		}

		return Vec::new();
	}

	pub fn is_legal(&mut self, x: usize, y: usize, dir: Direction) -> bool {
		match dir {
			Direction::North => {
				if (y - 1) <= 1 {
					println!("Moving {:?} from ({}, {}) is illegal.", dir, x, y);
					return false;
				}
			},
			Direction::East => {
				if (x + 1) >= (self.cap_x - 2) {
					println!("Moving {:?} from ({}, {}) is illegal.", dir, x, y);
					return false;
				}
			},
			Direction::South => {
				if (y + 1) >= (self.cap_y - 2) {
					println!("Moving {:?} from ({}, {}) is illegal.", dir, x, y);
					return false;
				}
			},
			Direction::West => {
				if (x - 1) <= 1 {
					println!("Moving {:?} from ({}, {}) is illegal.", dir, x, y);
					return false;
				}
			}
		}

		true
	}

	pub fn move_to(&mut self, x: usize, y: usize, dir: Direction) -> (usize, usize) {
		if self.is_legal(x, y, dir) {
			match dir {
				Direction::North => return (x, y - 1),
				Direction::East => return (x + 1, y),
				Direction::South => return (x, y + 1),
				Direction::West => return (x - 1, y),
			}
		}

		(x, y)
	}

	pub fn init(&mut self) -> Vec<u8> {
		let mut grid: Vec<u8> = Vec::with_capacity(self.cap_x * self.cap_y);

		for _ in 0..(self.cap_x * self.cap_y) {
			grid.push(0);
		}

		return grid;
	}
}
