use clap::{App, Arg};

pub mod cave;
pub mod config;
pub mod game;
pub mod screen;
pub mod window;

pub struct SdlContext {
	_image_context:			sdl2::image::Sdl2ImageContext,
	pub sdl_context:		sdl2::Sdl,
	pub ttf_context:		sdl2::ttf::Sdl2TtfContext,
}

impl SdlContext {
	fn init() -> SdlContext {
		SdlContext {
			_image_context: sdl2::image::init(sdl2::image::InitFlag::PNG)
				.expect("Init failed: SDL Image"),
			sdl_context: sdl2::init()
				.expect("Init failed: sdl_context"),
			ttf_context: sdl2::ttf::init()
				.expect("Init failed: ttf_context"),
		}
	}
}

struct Window<'sdl> {
	screen: &'sdl mut screen::Screen<'sdl>,
}

impl Window<'_> {
	fn new<'sdl>(screen: &'sdl mut screen::Screen<'sdl>) -> Window<'sdl> {
		Window {
			screen,
		}
	}

	fn launch(&mut self) {
		self.screen.game_loop();
	}
}

fn main() {
	const _CFG_FILE: &'static str = "cavegen.conf.toml";
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	let _matches = App::new("cavegen - simple cave generation.")
		.version(VERSION)
		.about("Generate a simple cave system.")
		.arg(Arg::with_name("config")
			.short("c")
			.long("config")
			.takes_value(true)
			.help("specify the path to a config file. Default: ./cavegen.conf.toml"))
		.arg(Arg::with_name("world")
			.short("w")
			.long("world")
			.takes_value(true)
			.help("Specify a world to load. Default: Generate a new world."))
		.get_matches();

	let mut c: config::Config = config::Config::new();
	if !c.load() {
		println!("Failed to load config from file.");
	}
	let (screen_size_x, screen_size_y) = (c.width, c.height);
	let mut cave = cave::Cave::new(screen_size_x as usize, screen_size_y as usize);

	cave.generate(50usize, 40usize, 1);
	cave.x = (screen_size_x / 2) as usize;
	cave.y = (screen_size_y / 2) as usize;
	cave.n = 0;

	let mut sdl_context = SdlContext::init();

	let mut screen = screen::Screen::new(c, &mut cave, &mut sdl_context);
	let mut window = Window::new(&mut screen);
	window.launch();

	println!("Exit cageven.");
}
