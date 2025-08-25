use macroquad::prelude::*;

pub struct SimpleRoom {
    pub left_exit: bool,
    pub right_exit: bool,
    pub top_exit: bool,
    pub bottom_exit: bool
}

pub fn draw_room(room: SimpleRoom, top_left: Vec2, scale: f32) {
  let room_inset = scale / 8.0;
  let exit_offset = room_inset * 2.0;
  let exit_size = scale - exit_offset * 2.0;
  draw_rectangle(top_left.x, top_left.y, scale, scale, BLACK);
  draw_rectangle(top_left.x + room_inset, top_left.y + room_inset, scale - exit_offset, scale - exit_offset, RED);
  if room.bottom_exit {
    draw_rectangle(top_left.x + exit_offset, top_left.y + scale - room_inset, exit_size, room_inset, GREEN);
  }
  if room.top_exit {
    draw_rectangle(top_left.x + exit_offset, top_left.y, exit_size, room_inset, GREEN);
  }
  if room.right_exit {
    draw_rectangle(top_left.x + scale - room_inset, top_left.y + exit_offset, room_inset, exit_size, GREEN);
  }
  if room.left_exit {
    draw_rectangle(top_left.x, top_left.y + exit_offset, room_inset, exit_size, GREEN);
  }
}