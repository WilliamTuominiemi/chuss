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

        draw_board();

        next_frame().await
    }
}

fn draw_board() {
    let block_size = screen_width() / 9.0;
    let block_offset = screen_width() / 18.0;

    for i in 0..8 {
        for j in 0..8 {
            let mut block_color: Color = WHITE;

            if i % 2 == 1 && j % 2 == 0 {
                block_color = BLACK;
            } else if i % 2 == 0 && j % 2 == 1 {
                block_color = BLACK;
            }

            draw_rectangle(
                i as f32 * block_size + block_offset,
                j as f32 * block_size + block_offset,
                block_size,
                block_size,
                block_color,
            );
        }
    }
}

fn draw_edges() {
    let characters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    let block_offset = screen_width() / 18.0;

    for n in numbers {
        draw_text(
            &n.to_string(),
            block_offset / 2.0,
            n as f32 * screen_height() / 9.0 - screen_height() / 9.0 / 2.0 + block_offset,
            30.0,
            BLACK,
        );
    }

    for c in 0..characters.len() {
        draw_text(
            &characters[c].to_string(),
            c as f32 * screen_width() / 9.0 + screen_width() / 9.0 / 2.0 + block_offset,
            screen_height() - block_offset / 2.0,
            30.0,
            BLACK,
        );
    }
}
