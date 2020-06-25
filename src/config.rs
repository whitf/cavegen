use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
	pub config_file:						String,
	pub height:								u32,	// y
	pub width:								u32,	// x
	pub map_tiles:							String,
	pub menu_bg:							String,
}

impl Config {
	pub fn new() -> Self {
		let config_file = String::from("cavegen.conf.toml");
		let height = 8u32;
		let width = 8u32;
		let map_tiles = String::from("assets/rpg_tiles_full.png");
		let menu_bg = String::from("assets/menu_bg.png");

		Config {
			config_file,
			height,
			width,
			map_tiles,
			menu_bg,
		}
	}

	pub fn load(&mut self) -> bool {
		let toml_content = fs::read_to_string(&self.config_file)
			.expect("Could not read toml config file.");

		let config:Config = toml::from_str(&toml_content)
			.expect("Could not create Config from toml.");
		
		*self = config;

		true
	}
}