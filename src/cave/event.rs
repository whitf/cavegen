use rusqlite::{Connection, NO_PARAMS, params};
use sdl2::image::{LoadTexture, SaveSurface};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

use crate::game;
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

pub fn create_new(cave: &mut cave::Cave) -> bool {
    println!("[cave.new]");

    cave.id = Uuid::new_v4();

    cave.generate(50usize, 40usize, 1);
    cave.x = (cave.screen_size_x / 2) as usize;
    cave.y = (cave.screen_size_y / 2) as usize;
    cave.n = 0;

    cave.menu_context = gfx::MenuContext::Game;
    true
}

pub fn load(cave: &mut cave::Cave) -> bool {
    let id = Uuid::parse_str("94324e4a-7919-4a2a-9af0-46d639fe9608").expect("Uuid::parse_str error");
    println!("[cave.load] loading id = {}", id.to_string());

    let dbfile: &Path = Path::new("./data");
    let mut dbfile = dbfile.join(id.to_string());
    dbfile.set_extension("game.db");

    let conn = Connection::open(dbfile)
        .expect("[load] Failed to open connection to dbfile.");

    let mut stmt = conn.prepare(
        "SELECT id, map_size_x, map_size_y, grid FROM level;",
    ).expect("[load] Failed prepare SELECT statment.");

    let levels = stmt.query_map(NO_PARAMS, |row| {
        let level_id = row.get(0).expect("Failed to get row(0) (id)");
        let map_size_x_int: i32 = row.get(1).expect("Failed to get row(1) (map_size_x)");
        let map_size_y_int: i32 = row.get(2).expect("Failed to get row(2) (map_size_y)");
        let grid: Vec<u8> = row.get(3).expect("Failed to get row(3) (grid)");

        Ok(cave::level::Level {
            id: level_id,
            grid,
            map_size_x: map_size_x_int as usize,
            map_size_y: map_size_y_int as usize,
            portal: Vec::new(),
        })
    }).expect("[load] Failed stmy.query_map");

    cave.level = Vec::new();

    for l in levels {
        println!("level = {:?}", l);

        cave.level.push(l.unwrap());
    }

    cave.menu_context = gfx::MenuContext::Game;
    true
}

pub fn save(cave: &mut cave::Cave) -> bool {
    println!("[cave.save]");

    if !game::util::dbinit(cave.id) {
        println!("Failed to initialize db/dbfile.");
        return false;
    }

    let dbfile: &Path = Path::new("./data");
    let mut dbfile = dbfile.join(cave.id.to_string());
    dbfile.set_extension("game.db");

    let conn = Connection::open(dbfile)
        .expect("[save] Failed to open dbfile.");

    for level in &cave.level {
        println!("attempt to insert level {} into db:", level.id);

        conn.execute(
            "REPLACE INTO level(id, map_size_x, map_size_y, grid) VALUES(?1, ?2, ?3, ?4);",
            params![level.id, level.map_size_x.to_string(), level.map_size_y.to_string(), level.grid],
        ).expect("[save] Failed to insert level data.");
    }

    cave.menu_context = gfx::MenuContext::Game;
    true
}

pub fn export(cave: &mut cave::Cave) -> bool {
    println!("[cave.export] export cave id = {:?}", cave.id);

    let fp: &Path = Path::new("./export/");
    let mut fp = fp.join(cave.id.to_string());
    fp.set_extension("png");

    let level: &level::Level = &cave.level[0];

    let mut img_surface: Surface = Surface::new((level.map_size_x * 64) as u32, (level.map_size_y * 64) as u32, PixelFormatEnum::RGB24).unwrap();
    img_surface.fill_rect(None, Color::RGBA(0, 0, 0, 255)).unwrap();

    let mut canvas = Canvas::from_surface(img_surface).expect("[export] Failed to create canvas from img_surface.");

    let png: &'static str = "/home/whit/workspace/cavegen/assets/simple_ground_32_32.png";
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
    println!(" . width = {}px", img_surface.width());
    println!(" . height = {}px", img_surface.height());

    img_surface.save(fp).expect("[export] Failed to save img_surface to file path.");
    
    cave.menu_context = gfx::MenuContext::Game;
    true
}
