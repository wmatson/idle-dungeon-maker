use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use macroquad::prelude::*;

pub mod room;

pub struct MapLevel<const W: usize, const H: usize> {
    pub rooms: [[Option<room::SimpleRoomDrawInfo>; W]; H],
}

// from_room, to_room
type RoomPredicate = fn(room::SimpleRoomDrawInfo, room::SimpleRoomDrawInfo) -> bool;

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

#[derive(Clone, Copy, Debug)]
pub struct TraversalInfo {
    pub depth: i32,
    pub row: isize,
    pub col: isize,
    pub room_info: room::SimpleRoomDrawInfo,
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
    const MAX_TRAVERSAL_VISITS: usize = W * H;

    pub fn draw(&self, top_left: Vec2, scale: f32) -> MapLevelDrawingCoords<W, H> {
        draw_rectangle(
            top_left.x,
            top_left.y,
            scale * W as f32,
            scale * H as f32,
            room::ROOM_BACKGROUND.with_alpha(0.8),
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

        let mut traversal_visit_count = 0;

        while !traversal_queue.is_empty() {
            if traversal_visit_count > Self::MAX_TRAVERSAL_VISITS {
                panic!(
                    "Traversal looped too many times, current queue: {:?}",
                    traversal_queue
                );
            }
            traversal_visit_count += 1;
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
                            });
                            already_visited.insert((new_row, new_col));
                        }
                    });
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
    ) -> Option<(Option<room::SimpleRoomDrawInfo>, Vec4, (usize, usize))> {
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
    use crate::map::{MapLevel, TraversalInfo, room::SimpleRoomDrawInfo, room::room_type};

    fn count_some_2d<const W: usize, const H: usize, T>(array2d: [[Option<T>; W]; H]) -> usize {
        array2d
            .iter()
            .flat_map(|x| x.iter().filter(|xinner| xinner.is_some()))
            .count()
    }

    #[test]
    fn test_room_rotation() {
        for room in room_type::ALL_TYPES {
            let l1 = room.rotate_left();
            let l2 = l1.rotate_left();
            let l3 = l2.rotate_left();
            let l4 = l3.rotate_left();

            let r1 = room.rotate_right();
            let r2 = r1.rotate_right();
            let r3 = r2.rotate_right();
            let r4 = r3.rotate_right();

            assert_eq!(l1, r3);
            assert_eq!(l2, r2);
            assert_eq!(room, l4);
            assert_eq!(room, r4);
        }

        let base_room = SimpleRoomDrawInfo {
            left_exit: true,
            right_exit: false,
            top_exit: false,
            bottom_exit: false,
            symbol: None,
        };

        assert!(!base_room.rotate_left().left_exit);
        assert!(!base_room.rotate_right().left_exit);

        assert!(base_room.rotate_left().bottom_exit);
        assert!(base_room.rotate_right().top_exit);
    }

    #[test]
    fn test_basic_traversal() {
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
        assert_eq!(count_some_2d(traversal_result), 3);

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

    #[test]
    fn test_traversal_disconnected_entrances() {
        let left_room = room_type::DEAD_END.rotate_right();
        let center_hall = room_type::HALL.rotate_left();
        let right_room = room_type::NO_EXIT;

        assert!(left_room.right_exit);
        assert!(center_hall.left_exit);
        assert!(center_hall.right_exit);
        assert!(!right_room.left_exit);

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
        assert!(traversal_result[0][2].is_none());

        assert_eq!(left_room, left_traversal.room_info);
        assert_eq!(left_traversal.depth, 0);
        assert_eq!(center_hall, center_traversal.room_info);
        assert_eq!(center_traversal.depth, 1);

        traversal_result = [[None; 3]];

        map.breadth_traverse(0, 2, |ti| {
            traversal_result[ti.row as usize][ti.col as usize] = Some(ti);
        });

        let right_traversal = traversal_result[0][2].unwrap();

        assert_eq!(right_room, right_traversal.room_info);
        assert_eq!(right_traversal.depth, 0);
        assert_eq!(count_some_2d(traversal_result), 1);
    }

    #[test]
    fn test_traversal_loops() {
        let enter_up_right = room_type::L;
        let enter_down_right = enter_up_right.rotate_right();
        let enter_down_left = enter_down_right.rotate_right();
        let enter_left_up = enter_down_left.rotate_right();

        let map = MapLevel {
            rooms: [
                [Some(enter_down_right), Some(enter_down_left)],
                [Some(enter_up_right), Some(enter_left_up)],
            ],
        };

        let mut traversal_result: [[Option<TraversalInfo>; 2]; 2] = [[None; 2]; 2];

        map.breadth_traverse(0, 0, |ti| {
            traversal_result[ti.row as usize][ti.col as usize] = Some(ti);
        });

        assert_eq!(count_some_2d(traversal_result), 4);
        assert_eq!(traversal_result[0][0].unwrap().depth, 0);
        assert_eq!(traversal_result[1][0].unwrap().depth, 1);
        assert_eq!(traversal_result[0][1].unwrap().depth, 1);
        assert_eq!(traversal_result[1][1].unwrap().depth, 2);
    }

    #[test]
    fn test_traversal_multiple_enter_close_disconnect() {
        let enter_up_right = room_type::L;
        let enter_down_right = enter_up_right.rotate_right();
        let enter_down_left = enter_down_right.rotate_right();
        let enter_left_up = enter_down_left.rotate_right();

        let map = MapLevel {
            rooms: [
                [
                    Some(enter_down_right),
                    Some(room_type::HALL.rotate_right()),
                    Some(enter_down_left),
                ],
                [
                    Some(enter_up_right),
                    Some(enter_up_right),
                    Some(enter_left_up),
                ],
            ],
        };

        let mut traversal_result: [[Option<TraversalInfo>; 3]; 2] = [[None; 3]; 2];

        map.breadth_traverse(0, 0, |ti| {
            traversal_result[ti.row as usize][ti.col as usize] = Some(ti);
        });

        assert_eq!(
            count_some_2d(traversal_result),
            6,
            "expected full traversal"
        );
        assert_eq!(traversal_result[0][0].unwrap().depth, 0);
        assert_eq!(traversal_result[1][0].unwrap().depth, 1);
        assert_eq!(traversal_result[0][1].unwrap().depth, 1);
        assert_eq!(traversal_result[0][2].unwrap().depth, 2);
        assert_eq!(traversal_result[1][2].unwrap().depth, 3);
        assert_eq!(traversal_result[1][1].unwrap().depth, 4);
    }

    #[test]
    fn test_traversal_open_dead_ends() {
        let left_room = room_type::DEAD_END.rotate_right();
        let center_hall = room_type::HALL.rotate_left();
        let right_crossing = room_type::CROSSING;

        assert!(left_room.right_exit);
        assert!(center_hall.left_exit);
        assert!(center_hall.right_exit);
        assert!(right_crossing.left_exit);

        let map = MapLevel {
            rooms: [
                [Some(left_room), Some(center_hall), Some(right_crossing)],
                [None; 3],
            ],
        };

        let mut traversal_result: [[Option<TraversalInfo>; 3]; 2] = [[None; 3]; 2];

        // shouldn't panic even though right_hall opens to the outside of the map and towards a non-room
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
        assert_eq!(right_crossing, right_traversal.room_info);
        assert_eq!(right_traversal.depth, 2);
    }
}
