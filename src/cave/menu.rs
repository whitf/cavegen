use sdl2::image::LoadTexture;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;

use crate::cave;

pub struct Menu {
	pub rect:					Rect,
}

impl Menu {
	pub fn new(width: u32, height: u32) -> Self {

		//println!("new rect {} x {}", width - 64, height - 64);
		Menu {
			rect: Rect::new(64, 64, width - 128, height - 128),
		}
	}

	pub fn draw(&mut self, cave: &mut cave::Cave, canvas: &mut WindowCanvas) {
		let mut menu_surface: Surface = Surface::new(self.rect.width(), self.rect.height(), PixelFormatEnum::RGB24).unwrap();
		menu_surface.fill_rect(None, Color::RGBA(128, 128, 128, 255)).unwrap();

		let png: &'static str = "/home/whit/workspace/cavegen/assets/menu_bg.png";
		let texture_creator = canvas.texture_creator();
		let menu_texture = texture_creator.load_texture(png)
			.expect("Failed to load texture from menu_bg.png");

		canvas.copy(&menu_texture, None, self.rect)
			.expect("[menu] Failed to copy menu rect to canvas.");



		// let texture_creator = canvas.texture_creator();
		// let mut menu_texture = texture_creator
		// 	.create_texture_from_surface(&menu_surface)
		// 	.expect("[menu] Error creating texture from surface.");
		// canvas.copy(&menu_texture, None, self.rect)
		// 	.expect("[menu] Error copying menubox to canvas.)");




	}
}
