use macroquad::prelude::*;
mod map;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 3;

#[macroquad::main("idle-dungeon-maker")]
async fn main() {
    let mut rooms = [[None; MAP_WIDTH]; MAP_HEIGHT];
    rooms[MAP_HEIGHT - 1][MAP_WIDTH / 2] = Some(map::SimpleRoom {
        top_exit: true,
        right_exit: true,
        left_exit: true,
        bottom_exit: false,
        symbol: Some('E'),
    });
    loop {
        let map = map::MapLevel { rooms };
        clear_background(LIGHTGRAY);

        let map_scale = screen_width() / 10.0;
        let coords = map.draw(
            Vec2::new(
                screen_width() / 2.0 - map_scale * MAP_WIDTH as f32 / 2.0,
                screen_height() / 2.0 - map_scale * MAP_HEIGHT as f32 / 2.0,
            ),
            map_scale,
        );

        let (mouse_x, mouse_y) = mouse_position();
        let bounds = coords.get_room(
            &map,
            Vec2 {
                x: mouse_x,
                y: mouse_y,
            },
        );
        bounds.inspect(|(room, found_bounds, (row, col))| {
            draw_rectangle_lines(
                found_bounds.x,
                found_bounds.y,
                map_scale,
                map_scale,
                2.0,
                GREEN,
            );
            if is_mouse_button_released(MouseButton::Left) && room.is_none() {
                rooms[*row][*col] = Some(map::SimpleRoom {
                    top_exit: false,
                    right_exit: false,
                    left_exit: false,
                    bottom_exit: false,
                    symbol: None,
                })
            }
        });

        next_frame().await
    }
}
