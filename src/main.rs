mod chess;
use chess::{Board, Piece, PieceType, Player};
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
    let board: Board = Board::start_position();

    let mut selected_square: (f32, f32) = (-1.0, -1.0);

    loop {
        clear_background(LIGHTGRAY);

        draw_edges();

        draw_board();

        selected_square = set_new_selected_square(selected_square);

        draw_selected_square(selected_square);

        draw_pieces(&board);

        next_frame().await
    }
}

fn set_new_selected_square(selected_square: (f32, f32)) -> (f32, f32) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_grid_pos_x = mouse_position().0;
        let mouse_grid_pos_y = mouse_position().1;

        let new_selected_square = (
            snap_to_grid(mouse_grid_pos_x),
            snap_to_grid(mouse_grid_pos_y),
        );

        if new_selected_square != selected_square {
            return new_selected_square;
        } else {
            return (-1.0, -1.0);
        }
    }

    return selected_square;
}

fn draw_selected_square(selected_square: (f32, f32)) {
    let block_size = screen_width() / 9.0;

    if selected_square != (-1.0, -1.0) {
        draw_rectangle(
            selected_square.0,
            selected_square.1,
            block_size,
            block_size,
            RED,
        );
    }
}

fn snap_to_grid(value: f32) -> f32 {
    let block_size = screen_width() / 9.0;
    let block_offset = screen_width() / 18.0;

    let index = ((value - block_offset) / block_size).floor();
    index * block_size + block_offset
}

fn draw_pieces(board: &Board) {
    let mut piece_index = 0;
    for piece in board.get_all_pieces() {
        let row = piece_index / 8;
        let col = piece_index % 8;
        let block_size = screen_width() / 9.0;
        let block_offset = screen_width() / 9.0 / 2.0;
        let piece_x = col as f32 * block_size + 5.0 + block_size / 2.0;
        let piece_y = row as f32 * block_size + block_size / 2.0;

        if let Some(piece) = piece {
            let color_string = format!("{:?}", piece.color);
            let piece_str = format!("{:?}", piece.kind);
            draw_text(
                &color_string,
                piece_x,
                piece_y + block_offset,
                30.0,
                DARKGRAY,
            );
            draw_text(
                &piece_str,
                piece_x,
                piece_y + block_offset * 1.5,
                30.0,
                DARKGRAY,
            );
        }

        piece_index += 1;
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
