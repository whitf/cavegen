use crate::cave;


// @TODO : All of these functions need to be updated to stay screen_width/2 or screen_height/2 away from x_min, y_min, x_max, y_max

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
