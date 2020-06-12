use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Direction {
	North,
	East,
	South,
	West,
	Up,
	Down,
}
