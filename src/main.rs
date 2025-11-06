use macroquad::prelude::*;

use crate::map::{TraversalInfo, room::SimpleRoomDrawInfo};
mod map;

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;

type MapInfo<T> = [[Option<T>; MAP_WIDTH]; MAP_HEIGHT];
type RoomsInfo = MapInfo<SimpleRoomDrawInfo>;

struct GameState {
    rooms: RoomsInfo,
    traversal_info: MapInfo<TraversalInfo>,
    entrance_rowcols: Vec<(usize, usize)>,
}

impl GameState {
    pub fn new(
        initial_entrance_row: usize,
        initial_entrance_col: usize,
        initial_room: SimpleRoomDrawInfo,
    ) -> Self {
        let mut game = GameState {
            rooms: [[None; MAP_WIDTH]; MAP_HEIGHT],
            traversal_info: [[None; MAP_WIDTH]; MAP_HEIGHT],
            entrance_rowcols: Vec::new(),
        };
        game.rooms[initial_entrance_row][initial_entrance_col] = Some(initial_room);
        game.traversal_info[initial_entrance_row][initial_entrance_col] = Some(TraversalInfo {
            depth: 0,
            row: initial_entrance_row as isize,
            col: initial_entrance_col as isize,
            room_info: initial_room,
        });
        game.entrance_rowcols
            .push((initial_entrance_row, initial_entrance_col));
        return game;
    }

    fn update_room(&mut self, row: usize, col: usize, new_room: Option<SimpleRoomDrawInfo>) {
        self.rooms[row][col] = new_room;
        // clear the traversal map
        self.traversal_info = [[None; MAP_WIDTH]; MAP_HEIGHT];
        // recalculate depths based on every entrance (only one should exist as of 2025-10-21 anyway), taking the lowest depth when two entrances can reach the same location
        for (e_row, e_col) in self.entrance_rowcols.iter() {
            self.get_map_level().breadth_traverse(*e_row, *e_col, |ti| {
                let info_slot = &mut self.traversal_info[ti.row as usize][ti.col as usize];
                *info_slot = Some(info_slot.map_or(ti, |existing| {
                    if existing.depth < ti.depth {
                        existing
                    } else {
                        ti
                    }
                }))
            });
        }
    }

    fn get_map_level(&self) -> map::MapLevel<MAP_WIDTH, MAP_HEIGHT> {
        return map::MapLevel { rooms: self.rooms };
    }
}

#[macroquad::main("idle-dungeon-maker")]
async fn main() {
    let entrance_row = MAP_HEIGHT - 1;
    let entrance_col = MAP_WIDTH / 2;

    let mut game = GameState::new(
        entrance_row,
        entrance_col,
        map::room::SimpleRoomDrawInfo {
            top_exit: true,
            right_exit: true,
            left_exit: true,
            bottom_exit: false,
            symbol: Some('E'),
        },
    );
    let mut current_creating_room_type: usize = 0;
    loop {
        let map = game.get_map_level();
        clear_background(LIGHTGRAY);

        let map_scale = screen_width() / 10.0;

        map::room::room_type::ALL_TYPES[current_creating_room_type]
            .draw(Vec2 { x: 20.0, y: 20.0 }, map_scale);
        draw_rectangle_lines(20.0, 20.0, map_scale, map_scale, 10.0, BLUE);
        if is_key_released(KeyCode::D) {
            current_creating_room_type =
                (current_creating_room_type + 1) % map::room::room_type::ALL_TYPES.len();
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
            game.traversal_info[*row][*col].inspect(|ti| {
                draw_multiline_text(
                    &ti.to_string(),
                    found_bounds.x + map_scale,
                    found_bounds.y + map_scale,
                    20.0,
                    None,
                    PURPLE,
                );
            });
            if is_mouse_button_released(MouseButton::Left)
                && room.is_none_or(|r| r.symbol.is_none_or(|s| s != 'E'))
            {
                let new_room = map::room::room_type::ALL_TYPES[current_creating_room_type].clone();
                game.update_room(*row, *col, Some(new_room));
            }
            if is_key_released(KeyCode::E) {
                game.update_room(*row, *col, room.map(|r| r.rotate_right()));
            }
            if is_key_released(KeyCode::Q) {
                game.update_room(*row, *col, room.map(|r| r.rotate_left()));
            }
        });

        next_frame().await
    }
}
