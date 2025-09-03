use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct SimpleRoom {
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

impl SimpleRoom {
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
}

pub struct MapLevel<const W: usize, const H: usize> {
    pub rooms: [[Option<SimpleRoom>; W]; H],
}

impl<const W: usize, const H: usize> MapLevel<W, H> {
    pub fn draw(&self, top_left: Vec2, scale: f32) {
        draw_rectangle(
            top_left.x,
            top_left.y,
            scale * W as f32,
            scale * H as f32,
            ROOM_BACKGROUND.with_alpha(0.8),
        );
        let mut y = top_left.y;
        for row in self.rooms {
            let mut x = top_left.x;
            for room in row {
                if room.is_some() {
                    room.unwrap().draw(Vec2::new(x, y), scale);
                }
                x += scale;
            }
            y += scale;
        }
    }
}
