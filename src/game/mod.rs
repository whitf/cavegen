use serde::Deserialize;

pub mod util;

#[derive(Debug, Deserialize)]
pub enum Direction {
	North,
	East,
	South,
	West,
	Up,
	Down,
}
