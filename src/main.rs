use macroquad::prelude::*;
mod map;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        map::draw_room(map::SimpleRoom {
            top_exit: true,
            right_exit: true,
            left_exit: true,
            bottom_exit: true
        }, Vec2::new(screen_width() / 2.0, screen_height() / 2.0), 20.0);

        next_frame().await
    }
}