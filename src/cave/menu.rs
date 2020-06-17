use sdl2::rect::Rect;

pub struct Menu {
	pub rect:					Rect,
}

impl Menu {
	pub fn new(width: u32, height: u32) -> Self {
		Menu {
			rect: Rect::new(0, 0, width, height),
		}
	}

	pub fn draw(&mut self) {
		println!("drawing menu...");
	}
}
