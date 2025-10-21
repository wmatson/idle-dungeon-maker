use macroquad::prelude::*;

use crate::map::SimpleRoomDrawInfo;
mod map;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;

type RoomsInfo = [[Option<SimpleRoomDrawInfo>; MAP_WIDTH]; MAP_HEIGHT];

struct GameState {
    pub rooms: RoomsInfo
}

impl GameState {
    fn update_room(&mut self, row: usize, col: usize, new_room: Option<SimpleRoomDrawInfo>) {
        self.rooms[row][col] = new_room;
    }

    fn get_map_level(&self) -> map::MapLevel<MAP_WIDTH, MAP_HEIGHT> {
        return map::MapLevel { rooms: self.rooms };
    }
}

#[macroquad::main("idle-dungeon-maker")]
async fn main() {
    let entrance_row = MAP_HEIGHT - 1;
    let entrance_col = MAP_WIDTH / 2;

    let mut game = GameState{
        rooms: [[None; MAP_WIDTH]; MAP_HEIGHT],
    };
    game.rooms[entrance_row][entrance_col] = Some(map::SimpleRoomDrawInfo {
        top_exit: true,
        right_exit: true,
        left_exit: true,
        bottom_exit: false,
        symbol: Some('E'),
    });
    let mut current_creating_room_type: usize = 0;
    loop {
        let map = game.get_map_level();
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
                let new_room = map::room_type::ALL_TYPES[current_creating_room_type].clone();
                game.update_room(*row, *col, Some(new_room));
            }
            if is_key_released(KeyCode::E) {
                game.update_room(*row, *col, room.map(|r| r.rotate_right()));
            }
            if is_key_released(KeyCode::Q) {
                game.update_room(*row, *col, room.map(|r| r.rotate_left()));
            }
        });

        if is_key_released(KeyCode::GraveAccent) {
            map.breadth_traverse(entrance_row, entrance_col, |x| println!("traversed {x}"));
        }

        next_frame().await
    }
}
