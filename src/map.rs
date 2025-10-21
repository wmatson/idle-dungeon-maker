use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleRoomDrawInfo {
    pub left_exit: bool,
    pub right_exit: bool,
    pub top_exit: bool,
    pub bottom_exit: bool,
    pub symbol: Option<char>,
}

const ROOM_BACKGROUND: Color = BLACK;
const ROOM_BODY: Color = RED;
const ROOM_EXIT: Color = RED;
const ROOM_SYMBOL: Color = GRAY;

impl SimpleRoomDrawInfo {
    pub fn draw(self, top_left: Vec2, scale: f32) {
        let room_inset = scale / 8.0;
        let exit_offset = room_inset * 2.0;
        let exit_size = scale - exit_offset * 2.0;
        draw_rectangle(top_left.x, top_left.y, scale, scale, ROOM_BACKGROUND);
        draw_rectangle(
            top_left.x + room_inset,
            top_left.y + room_inset,
            scale - exit_offset,
            scale - exit_offset,
            ROOM_BODY,
        );
        if self.bottom_exit {
            draw_rectangle(
                top_left.x + exit_offset,
                top_left.y + scale - room_inset,
                exit_size,
                room_inset,
                ROOM_EXIT,
            );
        }
        if self.top_exit {
            draw_rectangle(
                top_left.x + exit_offset,
                top_left.y,
                exit_size,
                room_inset,
                ROOM_EXIT,
            );
        }
        if self.right_exit {
            draw_rectangle(
                top_left.x + scale - room_inset,
                top_left.y + exit_offset,
                room_inset,
                exit_size,
                ROOM_EXIT,
            );
        }
        if self.left_exit {
            draw_rectangle(
                top_left.x,
                top_left.y + exit_offset,
                room_inset,
                exit_size,
                ROOM_EXIT,
            );
        }
        self.symbol.inspect(|sym| {
            let mut buffer = [0u8; 4];
            draw_text(
                sym.encode_utf8(&mut buffer),
                top_left.x + room_inset,
                top_left.y + scale - room_inset * 2.0,
                scale,
                ROOM_SYMBOL,
            );
        });
    }

    pub fn rotate_left(self) -> SimpleRoomDrawInfo {
        return SimpleRoomDrawInfo {
            top_exit: self.right_exit,
            right_exit: self.bottom_exit,
            bottom_exit: self.left_exit,
            left_exit: self.top_exit,
            symbol: self.symbol,
        };
    }

    pub fn rotate_right(self) -> SimpleRoomDrawInfo {
        return SimpleRoomDrawInfo {
            right_exit: self.top_exit,
            bottom_exit: self.right_exit,
            left_exit: self.bottom_exit,
            top_exit: self.left_exit,
            symbol: self.symbol,
        };
    }
}

pub mod room_type {
    use crate::map::SimpleRoomDrawInfo;

    pub const DEAD_END: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        top_exit: true,
        left_exit: false,
        right_exit: false,
        bottom_exit: false,
        symbol: None,
    };
    pub const HALL: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        top_exit: true,
        left_exit: false,
        right_exit: false,
        bottom_exit: true,
        symbol: None,
    };
    pub const L: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        top_exit: true,
        left_exit: false,
        right_exit: true,
        bottom_exit: false,
        symbol: None,
    };
    pub const T: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        left_exit: true,
        right_exit: true,
        top_exit: true,
        bottom_exit: false,
        symbol: None,
    };
    pub const CROSSING: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        left_exit: true,
        right_exit: true,
        top_exit: true,
        bottom_exit: true,
        symbol: None,
    };
    pub const NO_EXIT: SimpleRoomDrawInfo = SimpleRoomDrawInfo {
        left_exit: false,
        right_exit: false,
        top_exit: false,
        bottom_exit: false,
        symbol: None,
    };

    pub const ALL_TYPES: [SimpleRoomDrawInfo; 6] = [DEAD_END, L, HALL, T, CROSSING, NO_EXIT];
}

pub struct MapLevel<const W: usize, const H: usize> {
    pub rooms: [[Option<SimpleRoomDrawInfo>; W]; H],
}

// from_room, to_room
type RoomPredicate = fn(SimpleRoomDrawInfo, SimpleRoomDrawInfo) -> bool;

// pred, row, col
const TRAVERSAL_DIRS: [(RoomPredicate, isize, isize); 4] = [
    (
        |from_room, to_room| from_room.left_exit && to_room.right_exit,
        0,
        -1,
    ),
    (
        |from_room, to_room| from_room.right_exit && to_room.left_exit,
        0,
        1,
    ),
    (
        |from_room, to_room| from_room.top_exit && to_room.bottom_exit,
        -1,
        0,
    ),
    (
        |from_room, to_room| from_room.bottom_exit && to_room.top_exit,
        1,
        0,
    ),
];

#[derive(Clone, Copy)]
pub struct TraversalInfo {
    pub depth: i32,
    pub row: isize,
    pub col: isize,
    pub room_info: SimpleRoomDrawInfo,
}

impl fmt::Display for TraversalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "depth: {}, row: {}, col: {}",
            self.depth, self.row, self.col
        )
    }
}

pub struct MapLevelDrawingCoords<const W: usize, const H: usize> {
    coords: [[Vec4; W]; H],
}

impl<const W: usize, const H: usize> MapLevel<W, H> {
    pub fn draw(&self, top_left: Vec2, scale: f32) -> MapLevelDrawingCoords<W, H> {
        draw_rectangle(
            top_left.x,
            top_left.y,
            scale * W as f32,
            scale * H as f32,
            ROOM_BACKGROUND.with_alpha(0.8),
        );
        let mut y = top_left.y;
        let mut coords: [[Vec4; W]; H] = [[Vec4::ZERO; W]; H];
        for (row, room_row) in self.rooms.iter().enumerate() {
            let mut x = top_left.x;
            for (col, room) in room_row.iter().enumerate() {
                if room.is_some() {
                    room.unwrap().draw(Vec2::new(x, y), scale);
                }
                coords[row][col] = Vec4 {
                    x,
                    y,
                    z: x + scale,
                    w: y + scale,
                };
                x += scale;
            }
            y += scale;
        }
        return MapLevelDrawingCoords { coords };
    }

    pub fn breadth_traverse<TraversalFn>(
        &self,
        start_row: usize,
        start_col: usize,
        mut visitor: TraversalFn,
    ) where
        TraversalFn: FnMut(TraversalInfo),
    {
        if start_col >= W || start_row >= H {
            panic!("Start was outside map bounds! {start_col} -> [0,{W}), {start_row} -> [0, {H})")
        }
        let mut traversal_queue: VecDeque<TraversalInfo> = VecDeque::new();
        let mut already_visited: HashSet<(isize, isize)> = HashSet::new();

        self.rooms[start_row][start_col].inspect(|x| {
            traversal_queue.push_back(TraversalInfo {
                depth: 0,
                col: start_col as isize,
                row: start_row as isize,
                room_info: *x,
            })
        });
        already_visited.insert((start_row as isize, start_col as isize));

        while !traversal_queue.is_empty() {
            let current = traversal_queue
                .pop_front()
                .expect("Queue was unexpectedly empty");
            for (predicate, row_add, col_add) in TRAVERSAL_DIRS.iter() {
                let new_row = current.row + row_add;
                let new_col = current.col + col_add;
                if !already_visited.contains(&(new_row, new_col))
                    && new_row >= 0
                    && new_row < H as isize
                    && new_col >= 0
                    && new_col < W as isize
                {
                    self.rooms[new_row as usize][new_col as usize].inspect(|x| {
                        if predicate(current.room_info, *x) {
                            traversal_queue.push_back(TraversalInfo {
                                depth: current.depth + 1,
                                row: new_row,
                                col: new_col,
                                room_info: *x,
                            })
                        }
                    });
                    already_visited.insert((new_row, new_col));
                }
            }
            visitor(current);
        }
    }
}

impl<const W: usize, const H: usize> MapLevelDrawingCoords<W, H> {
    pub fn get_room(
        &self,
        level: &MapLevel<W, H>,
        point: Vec2,
    ) -> Option<(Option<SimpleRoomDrawInfo>, Vec4, (usize, usize))> {
        for (row, coord_row) in self.coords.iter().enumerate() {
            for (col, coord) in coord_row.iter().enumerate() {
                if point.x > coord.x && point.y > coord.y && point.x < coord.z && point.y < coord.w
                {
                    return Some((level.rooms[row][col], *coord, (row, col)));
                }
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_traversal() {
        use crate::map::{MapLevel, TraversalInfo, room_type};

        let left_room = room_type::DEAD_END.rotate_right();
        let center_hall = room_type::HALL.rotate_left();
        let right_room = room_type::DEAD_END.rotate_left();

        assert!(left_room.right_exit);
        assert!(center_hall.left_exit);
        assert!(center_hall.right_exit);
        assert!(right_room.left_exit);

        let map = MapLevel {
            rooms: [[Some(left_room), Some(center_hall), Some(right_room)]],
        };

        let mut traversal_result: [[Option<TraversalInfo>; 3]; 1] = [[None; 3]];

        map.breadth_traverse(0, 0, |ti| {
            traversal_result[ti.row as usize][ti.col as usize] = Some(ti);
        });

        for ti in traversal_result {
            assert!(!ti.is_empty())
        }

        let left_traversal = traversal_result[0][0].unwrap();
        let center_traversal = traversal_result[0][1].unwrap();
        let right_traversal = traversal_result[0][2].unwrap();

        assert_eq!(left_room, left_traversal.room_info);
        assert_eq!(left_traversal.depth, 0);
        assert_eq!(center_hall, center_traversal.room_info);
        assert_eq!(center_traversal.depth, 1);
        assert_eq!(right_room, right_traversal.room_info);
        assert_eq!(right_traversal.depth, 2);
    }
}
