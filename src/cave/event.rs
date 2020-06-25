use crate::cave;
use crate::cave::gfx;

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
    println!("[cave.export]");

    cave.menu_context = gfx::MenuContext::Game;
}
