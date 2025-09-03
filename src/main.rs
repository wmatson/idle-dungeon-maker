use macroquad::prelude::*;
mod map;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 3;

#[macroquad::main("idle-dungeon-maker")]
async fn main() {
    let mut rooms=  [[None; MAP_WIDTH]; MAP_HEIGHT];
    rooms[MAP_HEIGHT - 1][MAP_WIDTH / 2] = Some(map::SimpleRoom {
            top_exit: true,
            right_exit: true,
            left_exit: true,
            bottom_exit: false,
            symbol: Some('E')
        });
    let map = map::MapLevel {
        rooms
    };
    loop {
        clear_background(LIGHTGRAY);

        let map_scale = screen_width() / 10.0;
        map.draw(Vec2::new(screen_width() / 2.0 - map_scale * MAP_WIDTH as f32 / 2.0, screen_height() / 2.0 - map_scale * MAP_HEIGHT as f32 / 2.0), map_scale);

        next_frame().await
    }
}
