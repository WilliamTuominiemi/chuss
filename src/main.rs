use std::ffi::IntoStringError;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "chuss".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        draw_edges();

        next_frame().await
    }
}

fn draw_edges() {
    let characters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    for n in numbers {
        draw_text(
            &n.to_string(),
            0.0,
            n as f32 * screen_height() / 8.0 - screen_height() / 8.0 / 2.0,
            30.0,
            BLACK,
        );
    }

    for c in 0..characters.len() {
        draw_text(
            &characters[c].to_string(),
            c as f32 * screen_width() / 8.0 + screen_width() / 8.0 / 2.0,
            screen_height(),
            30.0,
            BLACK,
        );
    }
}
