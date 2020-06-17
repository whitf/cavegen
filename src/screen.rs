use sdl2;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;
use std::thread;
use std::time::Duration;

use crate::cave;
use crate::config;
use crate::SdlContext;

pub struct Screen<'a> {
	canvas:						WindowCanvas,
	cave:						&'a mut cave::Cave,
	png:						String,
	sdl_context:				&'a crate::SdlContext,
	screen_w:					u32,
	screen_h:					u32,
	tic:						Duration,
	tile_res:					u32,
} 

impl Screen<'_> {
	pub fn new<'a>(c: config::Config, cave: &'a mut cave::Cave, sdl_context: &'a mut SdlContext) -> Screen<'a> {
		let tile_res = 64u32;
		let fps = 60u64;
		let tic = Duration::from_millis(1000u64 / fps);
		let screen_w: u32 = c.width * tile_res;
		let screen_h: u32 = c.height * tile_res;
		let png = c.map_tiles;
		
		let video_subsystem = sdl_context.sdl_context.video()
			.expect("Init video failed: SDL video subsystem.");

		let bg_fill = Color::RGBA(0, 0, 0, 255);
		let window = video_subsystem.window("CaveGen v0.1.0", screen_w, screen_h)
			.position_centered().build().unwrap();
		let mut canvas = window.into_canvas()
			.accelerated().present_vsync().build()
			.expect("Could not create canvas from window.");
		canvas.set_draw_color(bg_fill);

		Screen {
			canvas,
			cave,
			png,
			sdl_context,
			screen_w,
			screen_h,
			tic,
			tile_res,
		}
	}

	pub fn game_loop(&mut self) {
		let context = &self.sdl_context.sdl_context;
		let texture_creator = self.canvas.texture_creator();
		let texture = texture_creator.load_texture(self.png.to_string())
			.expect("Failed to load game texture/map tiles.");

		let (origin_x, origin_y)  = (0i32, 0i32);
		let (mut game_cap_x, mut game_cap_y) = (self.screen_w/self.tile_res, self.screen_h/self.tile_res);
		
		// Make capacity even, helps with "centering" the map later.
		if game_cap_x % 2 != 0 {
			game_cap_x += 1;
		}

		if game_cap_y % 2 != 0 {
			game_cap_y += 1;
		}

		let mut event_pump = context.event_pump().unwrap();

		let mut db = cave::gfx::CaveBox::new(Rect::new(origin_x, origin_y, self.screen_w, self.screen_h));
		let mut menu_box = cave::menu::Menu::new(self.screen_w, self.screen_h);

		println!("self.screen_w = {}, self.screen_y = {})", self.screen_w, self.screen_h);
		println!("game+cap = ({}, {})", game_cap_x, game_cap_y);

		self.cave.screen_size_x = game_cap_x as usize;
		self.cave.screen_size_y = game_cap_y as usize;

		'gameloop: loop {
			for event in event_pump.poll_iter() {
				if !self.cave.process_event(event) {
					break 'gameloop;
				}
			}

			self.canvas.clear();
			db.draw(&texture, &mut self.cave, &mut self.canvas);

			if self.cave.menu_context == cave::gfx::MenuContext::Menu {
				menu_box.draw();
			}

			self.canvas.present();
			thread::sleep(self.tic);
		}
	}
}
