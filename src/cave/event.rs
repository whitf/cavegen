use sdl2::image::{LoadTexture, SaveSurface};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use std::path::Path;
use uuid::Uuid;

use crate::cave;
use crate::cave::{gfx, level, tile};

pub fn move_up(cave: &mut cave::Cave) {
    if cave.y >= 1 {
        if cave.y > (cave.screen_size_y / 2) {
            cave.y -= 1;
        }
    }
}

pub fn move_right(cave: &mut cave::Cave) {
    if cave.x < (cave.level[cave.n].map_size_x - 1) {
        if (cave.level[cave.n].map_size_x - cave.x) > (cave.screen_size_x / 2) {
            cave.x += 1;
        }
    }
}

pub fn move_down(cave: &mut cave::Cave) {
    if cave.y < (cave.level[cave.n].map_size_y - 1) {
        if (cave.level[cave.n].map_size_y - cave.y) > (cave.screen_size_y / 2) {
            cave.y += 1;
        }
    }
}

pub fn move_left(cave: &mut cave::Cave) {
    if cave.x >= 1 {
        if cave.x > (cave.screen_size_x / 2) {
            cave.x -= 1;
        }
    }
}

pub fn create_new(cave: &mut cave::Cave) {
    println!("[cave.new]");

    cave.id = Uuid::new_v4();

    cave.generate(50usize, 40usize, 1);
    cave.x = (cave.screen_size_x / 2) as usize;
    cave.y = (cave.screen_size_y / 2) as usize;
    cave.n = 0;

    cave.menu_context = gfx::MenuContext::Game;
}

pub fn load(cave: &mut cave::Cave) {
    println!("[cave.load]");

    cave.menu_context = gfx::MenuContext::Game;
}

pub fn save(cave: &mut cave::Cave) {
    println!("[cave.save]");

    cave.menu_context = gfx::MenuContext::Game;
}

pub fn export(cave: &mut cave::Cave) {
    println!("[cave.export] export cave id = {:?}", cave.id);

    let mut fp: &Path = Path::new("./export/");
    let mut fp = fp.join(cave.id.to_string());
    fp.set_extension("png");

    let level: &level::Level = &cave.level[0];

    let mut img_surface: Surface = Surface::new((level.map_size_x * 64) as u32, (level.map_size_y * 64) as u32, PixelFormatEnum::RGB24).unwrap();
    img_surface.fill_rect(None, Color::RGBA(0, 0, 0, 255)).unwrap();

    let mut canvas = Canvas::from_surface(img_surface).expect("[export] Failed to create canvas from img_surface.");

    let png: &'static str = "/home/whit/workspace/cavegen/assets/rpg_tiles_full.png";
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png).expect("[export] Failed to load tile textures.");

    let mut cell_cnt = 0;

    for y in 0..level.map_size_y {
        for x in 0..level.map_size_x {
            let index = (y * level.map_size_x) + x;
            let my_tile = tile::get_rect(level.grid[index]);
            let target_rect = Rect::new(
                (x * 64) as i32,
                (y * 64) as i32,
                64u32,
                64u32
            );

            canvas.copy(&texture, my_tile, target_rect)
                .expect("[export] Failed to copy tile into canvas.");

            cell_cnt += 1;
        }
    }

    img_surface = canvas.into_surface();
    
    println!("img_surface:");
    println!(" . copied {} cells", cell_cnt);
    println!(" . width = {}", img_surface.width());
    println!(" . height = {}", img_surface.height());

    img_surface.save(fp);
    cave.menu_context = gfx::MenuContext::Game;
}
