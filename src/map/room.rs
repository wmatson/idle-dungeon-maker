use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimpleRoomDrawInfo {
    pub left_exit: bool,
    pub right_exit: bool,
    pub top_exit: bool,
    pub bottom_exit: bool,
    pub symbol: Option<char>,
}

pub const ROOM_BACKGROUND: Color = BLACK;

pub const ROOM_BODY: Color = RED;

pub const ROOM_EXIT: Color = RED;

pub const ROOM_SYMBOL: Color = GRAY;

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
    use crate::map::room::SimpleRoomDrawInfo;

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
