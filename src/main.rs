use macroquad::prelude::*;
mod map;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;

#[macroquad::main("idle-dungeon-maker")]
async fn main() {
    let mut rooms = [[None; MAP_WIDTH]; MAP_HEIGHT];
    rooms[MAP_HEIGHT - 1][MAP_WIDTH / 2] = Some(map::SimpleRoomDrawInfo {
        top_exit: true,
        right_exit: true,
        left_exit: true,
        bottom_exit: false,
        symbol: Some('E'),
    });
    let mut current_creating_room_type: usize = 0;
    loop {
        let map = map::MapLevel { rooms };
        clear_background(LIGHTGRAY);

        let map_scale = screen_width() / 10.0;

        map::room_type::ALL_TYPES[current_creating_room_type]
            .draw(Vec2 { x: 20.0, y: 20.0 }, map_scale);
        draw_rectangle_lines(20.0, 20.0, map_scale, map_scale, 10.0, BLUE);
        if is_key_released(KeyCode::D) {
            current_creating_room_type =
                (current_creating_room_type + 1) % map::room_type::ALL_TYPES.len();
        }

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
            if is_mouse_button_released(MouseButton::Left)
                && room.is_none_or(|r| r.symbol.is_none_or(|s| s != 'E'))
            {
                rooms[*row][*col] =
                    Some(map::room_type::ALL_TYPES[current_creating_room_type].clone())
            }
            if is_key_released(KeyCode::E) {
                rooms[*row][*col] = room.map(|r| r.rotate_right())
            }
            if is_key_released(KeyCode::Q) {
                rooms[*row][*col] = room.map(|r| r.rotate_left())
            }
        });

        next_frame().await
    }
}
